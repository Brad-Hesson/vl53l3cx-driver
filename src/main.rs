#![no_std]
#![no_main]

mod led_display;
mod mutable_mutex;

use core::f64;
use core::ptr;
use core::slice;

use led_display::LedDisplay;
use mutable_mutex::MutableMutex;

use paste::paste;

use cortex_m;
use cortex_m_rt::entry;
use hal::{
    delay::Delay,
    device,
    gpio::{Output, Pin, PushPull, Speed, H8, L8},
    i2c, interrupt, pac,
    prelude::*,
    stm32::Interrupt,
    timer::{Event, Timer},
};
use panic_probe as _;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32l4xx_hal as hal;

type PA<const N: u8> = Pin<Output<PushPull>, L8, 'A', N>;
static TIM7_MUT: MutableMutex<Timer<device::TIM7>> = MutableMutex::new();
type Display =
    LedDisplay<PA<0>, PA<7>, PA<6>, Pin<Output<PushPull>, H8, 'A', 8>, PA<4>, PA<3>, PA<1>>;
static LED_DISPLAY_MUT: MutableMutex<Display> = MutableMutex::new();

#[entry]
fn main() -> ! {
    // ---------intialize and test the real-time target printing----------
    rtt_init_print!();
    rprintln!("Hello World");

    // ---------configure the peripheral access structures----------
    let device_peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut rcc = device_peripherals.RCC.constrain();
    let mut flash = device_peripherals.FLASH.constrain();
    let mut pwr = device_peripherals.PWR.constrain(&mut rcc.apb1r1);
    let sys_clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);
    let mut delay = Delay::new(cortex_peripherals.SYST, sys_clocks);

    // ---------configure the gpio ports and gpio pin macro----------
    let mut gpioa = device_peripherals.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = device_peripherals.GPIOB.split(&mut rcc.ahb2);
    macro_rules! gpio_pin {
        (push_pull_output : p $port:ident $pin:literal) => {
            paste!([<gpio $port>].[<p $port $pin>].into_push_pull_output(&mut [<gpio $port>].moder, &mut [<gpio $port>].otyper))
        };
        (open_drain_output : p $port:ident $pin:literal) => {
            paste!([<gpio $port>].[<p $port $pin>].into_open_drain_output(&mut [<gpio $port>].moder, &mut [<gpio $port>].otyper))
        };
        (open_drain_alternate : p $port:ident $pin:literal) => {
            paste!([<gpio $port>].[<p $port $pin>].into_alternate_open_drain(&mut [<gpio $port>].moder, &mut [<gpio $port>].otyper, &mut [<gpio $port>].afrh))
        };
    }

    // ---------configure the led display----------
    let display = LedDisplay::new(
        gpio_pin!(push_pull_output : p a 0),
        gpio_pin!(push_pull_output : p a 7),
        gpio_pin!(push_pull_output : p a 6),
        gpio_pin!(push_pull_output : p a 8),
        gpio_pin!(push_pull_output : p a 4),
        gpio_pin!(push_pull_output : p a 3),
        gpio_pin!(push_pull_output : p a 1),
    );
    LED_DISPLAY_MUT.critical_initialize(display).unwrap();

    // ---------configure the display timer----------
    let mut timer = Timer::tim7(
        device_peripherals.TIM7,
        3.kHz(),
        sys_clocks,
        &mut rcc.apb1r1,
    );
    timer.listen(Event::TimeOut);
    TIM7_MUT.critical_initialize(timer).unwrap();
    unsafe { device::NVIC::unmask(Interrupt::TIM7) };

    // ---------configure the ranging sensor----------
    let xshut_p = gpio_pin!(open_drain_output : p b 7);
    let mut scl_p = gpio_pin!(open_drain_alternate : p a 9).set_speed(Speed::VeryHigh);
    let mut sda_p = gpio_pin!(open_drain_alternate : p a 10).set_speed(Speed::VeryHigh);
    scl_p.internal_pull_up(&mut gpioa.pupdr, true);
    sda_p.internal_pull_up(&mut gpioa.pupdr, true);
    let i2c = i2c::I2c::i2c1(
        device_peripherals.I2C1,
        (scl_p, sda_p),
        i2c::Config::with_timing(0x00707CBB),
        &mut rcc.apb1r1,
    );
    let mut sensor = vl53l3cx_driver::VL53L3CX::new(i2c, 0x52, xshut_p);

    // ---------initialize the ranging sensor----------
    sensor.enable();
    sensor.wait_device_booted(&mut delay).unwrap();
    rprintln!("Model ID: 0x{:02X}", sensor.read_byte(0x010F).unwrap());
    rprintln!("Module Type: 0x{:02X}", sensor.read_byte(0x0110).unwrap());
    let uid = sensor.get_uid(&mut delay).unwrap();
    let puid = ptr::addr_of!(uid);
    let bytes = unsafe { slice::from_raw_parts(puid as *const u8, 8) };
    rprint!("Device UID: ");
    for byte in bytes {
        rprint!("0x{:02X} ", byte);
    }
    rprintln!();
    sensor.data_init(&mut delay).unwrap();
    sensor.set_distance_mode(3).unwrap();

    // ---------run the main loop----------
    loop {
        sensor.start_measurement().unwrap();
        sensor.wait_measurement_data_ready(&mut delay).unwrap();
        let data = sensor.get_multiranging_data().unwrap();
        for i in 0..data.NumberOfObjectsFound {
            let rd = data.RangeData[i as usize];
            rprintln!(
                "status={}, D={}mm, Signal={} Mcps, Ambient={} Mcps",
                rd.RangeStatus,
                rd.RangeMilliMeter,
                rd.AmbientRateRtnMegaCps as f32 / 65536.0,
                rd.AmbientRateRtnMegaCps as f32 / 65536.0
            );
        }
        let num = data.RangeData[0].RangeMilliMeter as f64 / 1000.0 * 10.0;
        let bar = 7 - ((num - (num as u8) as f64) * 8.0) as usize;
        let mut num = num as usize;
        num %= 10;
        LED_DISPLAY_MUT
            .critical_modify(|display| {
                if data.RangeData[0].RangeStatus == 0 {
                    display.display_number(num as u8);
                    for row in bar..8 {
                        for col in 0..8 {
                            display.buffer[row * 8 + col] = !!!display.buffer[row * 8 + col];
                        }
                    }
                } else {
                    display.buffer = [false; 64];
                }
            })
            .unwrap();
    }
}

#[interrupt]
fn TIM7() {
    cortex_m::interrupt::free(|cs| {
        LED_DISPLAY_MUT
            .modify(cs, |display| display.flush_next_pin())
            .unwrap();
        TIM7_MUT
            .modify(cs, |timer| timer.clear_interrupt(Event::TimeOut))
            .unwrap();
    });
}

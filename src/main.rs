#![no_std]
#![no_main]

mod led_display;
mod mutable_mutex;

use core::ptr;
use core::slice;

use led_display::LedDisplay;
use mutable_mutex::MutableMutex;
use vl53l3cx_driver;

use paste::paste;

use cortex_m;
use cortex_m_rt::entry;
use hal::{
    delay::Delay,
    device,
    gpio::{Output, Pin, PushPull, Speed, L8},
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
static LED_DISPLAY_MUT: MutableMutex<LedDisplay<PA<0>, PA<7>, PA<6>, PA<5>, PA<4>, PA<3>, PA<1>>> =
    MutableMutex::new();

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
    let clock = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

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

    // ---------configure the delay clock----------
    let mut delay = Delay::new(cortex_peripherals.SYST, clock);

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
    sensor.enable();
    rprintln!("Model ID: 0x{:02X}", sensor.read_byte(0x010F));
    rprintln!("Module Type: 0x{:02X}", sensor.read_byte(0x0110));
    let uid = sensor.get_uid(&mut delay);
    let puid = ptr::addr_of!(uid);
    let bytes = unsafe { slice::from_raw_parts(puid as *const u8, 8) };
    rprint!("Device UID: ");
    for byte in bytes {
        rprint!("0x{:02X} ", byte);
    }
    rprintln!();
    sensor.wait_device_booted(&mut delay);
    rprintln!("booted");
    sensor.data_init(&mut delay);
    rprintln!("initialized");
    loop {
        sensor.start_measurement();
        // while !!!sensor.get_measurement_data_ready() {}
        sensor.wait_measurement_data_ready();
        let data = sensor.get_multiranging_data();
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
    }

    // ---------configure the led display----------
    let display = LedDisplay::new(
        gpio_pin!(push_pull_output : p a 0),
        gpio_pin!(push_pull_output : p a 7),
        gpio_pin!(push_pull_output : p a 6),
        gpio_pin!(push_pull_output : p a 5),
        gpio_pin!(push_pull_output : p a 4),
        gpio_pin!(push_pull_output : p a 3),
        gpio_pin!(push_pull_output : p a 1),
    );
    LED_DISPLAY_MUT.critical_initialize(display).unwrap();

    // ---------configure the display timer----------
    let mut timer = Timer::tim7(device_peripherals.TIM7, 3.kHz(), clock, &mut rcc.apb1r1);
    timer.listen(Event::TimeOut);
    TIM7_MUT.critical_initialize(timer).unwrap();
    unsafe { device::NVIC::unmask(Interrupt::TIM7) };

    // ---------run the main loop----------
    let mut a = 0;
    loop {
        LED_DISPLAY_MUT
            .critical_modify(|display| {
                display.display_number(a % 10);
            })
            .unwrap();
        delay.delay_ms(1000u32);
        a += 1;
    }
}

#[interrupt]
fn TIM7() {
    cortex_m::interrupt::free(|cs| {
        TIM7_MUT
            .modify(cs, |timer| timer.clear_interrupt(Event::TimeOut))
            .unwrap();
        LED_DISPLAY_MUT.modify(cs, |display| display.flush_next_pin())
    })
    .unwrap();
}

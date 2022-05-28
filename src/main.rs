#![no_std]
#![no_main]

mod led_display;
mod mutable_mutex;

use led_display::LedDisplay;
use mutable_mutex::MutableMutex;

use paste::paste;

use cortex_m;
use cortex_m_rt::entry;
use hal::{
    delay::Delay,
    device::{NVIC, TIM7},
    gpio::{Output, Pin, PushPull, L8},
    interrupt, pac,
    prelude::*,
    stm32::Interrupt,
    timer::{Event, Timer},
};
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal as hal;

type PA<const N: u8> = Pin<Output<PushPull>, L8, 'A', N>;
static TIM7_DATA: MutableMutex<(
    Timer<TIM7>,
    LedDisplay<PA<0>, PA<7>, PA<6>, PA<5>, PA<4>, PA<3>, PA<1>>,
)> = MutableMutex::new();

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World");

    let device_peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

    let mut rcc = device_peripherals.RCC.constrain();
    let mut flash = device_peripherals.FLASH.constrain();
    let mut pwr = device_peripherals.PWR.constrain(&mut rcc.apb1r1);
    let clock = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = device_peripherals.GPIOA.split(&mut rcc.ahb2);
    // let gpiob = device_peripherals.GPIOB.split(&mut rcc.ahb2);
    macro_rules! push_pull_pin {
        (p $bank:ident $pin:literal) => {
            paste!([<gpio $bank>].[<p $bank $pin>].into_push_pull_output(&mut [<gpio $bank>].moder, &mut [<gpio $bank>].otyper))
        };
    }

    // let mut user_led = push_pull_pin!(p b 3);

    let display = LedDisplay::new(
        push_pull_pin!(p a 0),
        push_pull_pin!(p a 7),
        push_pull_pin!(p a 6),
        push_pull_pin!(p a 5),
        push_pull_pin!(p a 4),
        push_pull_pin!(p a 3),
        push_pull_pin!(p a 1),
    );
    let mut timer = Timer::tim7(device_peripherals.TIM7, 4.kHz(), clock, &mut rcc.apb1r1);
    timer.listen(Event::TimeOut);
    TIM7_DATA.critical_initialize((timer, display)).unwrap();
    unsafe {
        NVIC::unmask(Interrupt::TIM7);
    }

    let mut delay = Delay::new(cortex_peripherals.SYST, clock);

    let mut a = 1;
    loop {
        TIM7_DATA
            .critical_modify(|(_, ref mut display)| {
                display.buffer[a % 64] = true;
                display.buffer[(a + 16) % 64] = false;
            })
            .unwrap();
        delay.delay_ms(10u32);
        a += 1;
    }
}

#[interrupt]
fn TIM7() {
    TIM7_DATA
        .critical_modify(|(ref mut timer, ref mut display)| {
            timer.clear_interrupt(Event::TimeOut);
            display.flush_next_pin();
        })
        .unwrap();
}

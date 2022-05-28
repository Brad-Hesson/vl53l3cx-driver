#![allow(dead_code)]

use hal::prelude::OutputPin;
use stm32l4xx_hal as hal;

pub struct LedDisplay<EN, C1, C2, C3, R1, R2, R3> {
    en_p: EN,
    c1_p: C1,
    c2_p: C2,
    c3_p: C3,
    r1_p: R1,
    r2_p: R2,
    r3_p: R3,
    pub buffer: [bool; 64],
    display_index: usize,
}

impl<EN, C1, C2, C3, R1, R2, R3> LedDisplay<EN, C1, C2, C3, R1, R2, R3> {
    pub fn new(en_p: EN, c1_p: C1, c2_p: C2, c3_p: C3, r1_p: R1, r2_p: R2, r3_p: R3) -> Self {
        Self {
            en_p,
            c1_p,
            c2_p,
            c3_p,
            r1_p,
            r2_p,
            r3_p,
            buffer: [false; 64],
            display_index: 0,
        }
    }
    pub fn flush_next_pin(&mut self)
    where
        EN: OutputPin,
        C1: OutputPin,
        C2: OutputPin,
        C3: OutputPin,
        R1: OutputPin,
        R2: OutputPin,
        R3: OutputPin,
    {
        self.en_p.set_low().ok();
        if self.buffer[self.display_index] {
            let bs = [1, 2, 4, 8, 16, 32].map(|v| self.display_index / v % 2 == 1);
            self.r1_p.set_state(bs[0].into()).ok();
            self.r2_p.set_state(bs[1].into()).ok();
            self.r3_p.set_state(bs[2].into()).ok();
            self.c1_p.set_state(bs[3].into()).ok();
            self.c2_p.set_state(bs[4].into()).ok();
            self.c3_p.set_state(bs[5].into()).ok();
            self.en_p.set_high().ok();
        }
        self.display_index += 1;
        self.display_index %= 64;
    }
}

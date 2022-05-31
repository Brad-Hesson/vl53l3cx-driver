#![allow(dead_code)]

use stm32l4xx_hal::prelude::OutputPin;
mod graphics;

use graphics::*;

pub struct LedDisplay<EN, C1, C2, C3, R1, R2, R3>
where
    EN: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
    C3: OutputPin,
    R1: OutputPin,
    R2: OutputPin,
    R3: OutputPin,
{
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

impl<EN, C1, C2, C3, R1, R2, R3> LedDisplay<EN, C1, C2, C3, R1, R2, R3>
where
    EN: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
    C3: OutputPin,
    R1: OutputPin,
    R2: OutputPin,
    R3: OutputPin,
{
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
    pub fn display_number(&mut self, n: u8) {
        self.buffer = match n {
            0 => BUF_0,
            1 => BUF_1,
            2 => BUF_2,
            3 => BUF_3,
            4 => BUF_4,
            5 => BUF_5,
            6 => BUF_6,
            7 => BUF_7,
            8 => BUF_8,
            9 => BUF_9,
            _ => unimplemented!(),
        }
    }
    pub fn flush_next_pin(&mut self) {
        self.en_p.set_low().ok();
        if self.buffer[self.display_index] {
            let bs = [0b1, 0b10, 0b100, 0b1000, 0b10000, 0b100000]
                .map(|mask| self.display_index & mask != 0);
            self.c1_p.set_state(bs[0].into()).ok();
            self.c2_p.set_state(bs[1].into()).ok();
            self.c3_p.set_state(bs[2].into()).ok();
            self.r1_p.set_state(bs[3].into()).ok();
            self.r2_p.set_state(bs[4].into()).ok();
            self.r3_p.set_state(bs[5].into()).ok();
            self.en_p.set_high().ok();
        }
        self.display_index += 1;
        self.display_index %= 64;
    }
    pub fn shift_right(&mut self, n: usize) {
        self.buffer[0..8].rotate_right(n);
        self.buffer[8..16].rotate_right(n);
        self.buffer[16..24].rotate_right(n);
        self.buffer[24..32].rotate_right(n);
        self.buffer[32..40].rotate_right(n);
        self.buffer[40..48].rotate_right(n);
        self.buffer[48..56].rotate_right(n);
        self.buffer[56..64].rotate_right(n);
    }
    pub fn shift_left(&mut self, n: usize) {
        self.buffer[0..8].rotate_left(n);
        self.buffer[8..16].rotate_left(n);
        self.buffer[16..24].rotate_left(n);
        self.buffer[24..32].rotate_left(n);
        self.buffer[32..40].rotate_left(n);
        self.buffer[40..48].rotate_left(n);
        self.buffer[48..56].rotate_left(n);
        self.buffer[56..64].rotate_left(n);
    }
    pub fn shift_up(&mut self, n: usize) {
        self.buffer.rotate_left(n * 8);
    }
    pub fn shift_down(&mut self, n: usize) {
        self.buffer.rotate_right(n * 8);
    }
}

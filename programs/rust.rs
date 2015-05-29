// Rust on BareMetal - Tested with Rust 0.13.0-nightly
// Adapted from https://github.com/charliesome/rustboot and
// https://github.com/charliesome/rustboot/pull/25
// rustc -O --crate-type lib -o rust.o --emit obj rust.rs
// ld -T app.ld -o rust.app rust.o


#![feature(custom_attribute)]
#![feature(no_std)]

#![no_std]

#![feature(lang_items)]

#![feature(core)]

use core::prelude::*;

extern crate core;

#[derive(Copy,Clone)]
enum Color {
    Black       = 0,
    Red         = 1,
    Green       = 2,
    Blue        = 3,
    Yellow      = 4,
    Purple      = 5,
    Cyan        = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightRed    = 9,
    LightGreen  = 10,
    LightBlue   = 11,
    LightYellow = 12,
    LightPurple = 13,
    LightCyan   = 14,
    White       = 15,
}

struct IntRange {
    cur: u64,
    max: u64
}

impl IntRange {
    fn next(&mut self) -> Option<u64> {
        if self.cur < self.max {
            self.cur += 1;
            Option::Some(self.cur - 1)
        } else {
            Option::None
        }
    }
}

fn range(lo: u64, hi: u64) -> IntRange {
    IntRange { cur: lo, max: hi }
}

fn clear_screen(background: Color) {
    let mut r = range(0, 80 * 25);
    loop {
        match r.next() {
            Option::Some(x) => {
                unsafe {
                   *((0xb8000 + x * 2) as *mut u16) = (background as u16) << 12;
                }
            },
            Option::None => {break}
        }
    }
}

#[no_mangle]
pub fn main() {
    clear_screen(Color::LightRed);
}

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


#[no_mangle]
pub fn main() {
    clear_screen(Color::LightRed);
}


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

fn clear_screen(background: Color) {
    for x in 0 .. 80*25 {
        unsafe {
           *((0xb8000 + x * 2) as *mut u16) = (background as u16) << 12;
        }
    }
}

#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;

mod logic;
mod utils;

#[entry]
fn main() -> Status {
    logic::main_loop()
}

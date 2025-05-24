#![no_main]
#![no_std]

extern crate alloc;

mod logic;

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    logic::main_eventloop().unwrap()
}

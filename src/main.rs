#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;

mod utils;
mod logic;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    logic::main_loop(&mut system_table)
        .status()
}
#![no_main]
#![no_std]

extern crate alloc;

mod utils;
mod logic;

use uefi::prelude::*;
use uefi::proto::console::text::Output;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table)
        .expect("Cant init services");
    
    let bt = system_table.boot_services();
    let stdout = utils::protocols::open_scoped::<Output>(&bt);
    logic::main_loop(stdout)
}


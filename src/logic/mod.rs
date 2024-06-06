use uefi::proto::console::text::Output;
use uefi::Result;
use uefi::table::{Boot, SystemTable};
use crate::logic::draw::draw_fetch;
use crate::utils;

mod draw;

pub fn main_loop(mut system_table: &mut SystemTable<Boot>) -> Result {
    uefi::helpers::init(&mut system_table)?;
    let boot_services = system_table.boot_services();
    let runtime_services = system_table.runtime_services();
    let stdout = utils::protocols::open_scoped::<Output>(&boot_services);
    draw_fetch(stdout, runtime_services);

    loop {
        todo!("keyboard input events for loop")
    }
}
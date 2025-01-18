use uefi::proto::console::text::Output;
use uefi::table::{Boot, SystemTable};

use crate::logic::draw::{mem, menu};
use crate::utils;

mod draw;

pub(crate) fn main_loop(mut system_table: &mut SystemTable<Boot>) -> ! {
    uefi::helpers::init(&mut system_table).expect("Cant init helpers");

    let boot_services = system_table.boot_services();
    let runtime_services = system_table.runtime_services();
    let mut stdout = utils::protocols::open_scoped::<Output>(&boot_services);

    menu::draw(&mut stdout, runtime_services, boot_services);
    loop {}
}

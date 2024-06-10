use uefi::proto::console::text::{Input, Key, Output, ScanCode};
use uefi::{println, Result};
use uefi::table::{Boot, SystemTable};
use crate::logic::draw::menu;
use crate::utils;

mod draw;

pub fn main_loop(mut system_table: &mut SystemTable<Boot>) -> Result {
    uefi::helpers::init(&mut system_table)?;
    let boot_services = system_table.boot_services();
    let runtime_services = system_table.runtime_services();
    let mut stdout = utils::protocols::open_scoped::<Output>(&boot_services);
    let mut stdin = utils::protocols::open_scoped::<Input>(&boot_services);

    menu::draw(&mut stdout, runtime_services);
    loop {
        if let Some(key) = stdin.read_key()? {
            match key {
                Key::Printable(c) => {
                    match char::from(c) {
                        '1' => println!("R"),
                        _ => {}
                    }
                }
                Key::Special(ScanCode::ESCAPE) => {
                    menu::draw(&mut stdout, runtime_services);
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }
}
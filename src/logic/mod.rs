use crate::{logic::draw::menu_page, utils};
use draw::{cpu_page, mem_page};
use info::{cpu::CpuInfo, date::Date, mem::MemInfo};
use uefi::{
    proto::console::{
        gop::GraphicsOutput,
        text::{Input, Key, Output, ScanCode},
    },
    Status,
};

mod draw;
mod info;

pub fn main_eventloop() -> Status {
    uefi::helpers::init().unwrap();

    let cpu_info = CpuInfo::get();
    let date_info = Date::get().unwrap();
    let mem_info = MemInfo::get().unwrap();

    let mut stdout = utils::open_scoped::<Output>().unwrap();
    let mut stdin = utils::open_scoped::<Input>().unwrap();

    // Tests with minimal (80x25) resolution
    // let modes_vec = stdout.modes().enumerate().collect::<Vec<_>>();
    // stdout.set_mode(modes_vec[0].1).unwrap();

    menu_page::draw(&mut stdout, date_info).unwrap();

    loop {
        if let Some(key) = stdin.read_key().unwrap() {
            match key {
                Key::Special(ScanCode::DELETE) => {
                    uefi::boot::stall(700_000);
                    break;
                }
                Key::Special(ScanCode::ESCAPE) => menu_page::draw(&mut stdout, date_info).unwrap(),
                Key::Special(ScanCode::FUNCTION_1) => (), /* PCI */
                Key::Special(ScanCode::FUNCTION_2) => cpu_page::draw(&cpu_info),
                Key::Special(ScanCode::FUNCTION_3) => mem_page::draw(&mem_info),
                _ => {}
            }
        }
    }

    Status::SUCCESS
}

use crate::{logic::draw::menu_page, utils};
use info::{cpu::CpuInfo, date::Date, mem::MemInfo};
use uefi::proto::{console::text::Output, network::snp::SimpleNetwork};

mod draw;
mod info;

pub fn main_eventloop() -> ! {
    uefi::helpers::init().unwrap();

    let cpu_info = CpuInfo::get();
    let date_info = Date::get().unwrap();
    let mem_info = MemInfo::get().unwrap();

    let mut stdout = utils::protocols::open_scoped::<Output>().unwrap();
    let net = utils::protocols::open_scoped::<SimpleNetwork>().unwrap();

    _ = menu_page::draw(&mut stdout, &net, date_info).unwrap();

    loop {}
}

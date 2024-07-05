use alloc::format;
use uefi::{print, println};
use uefi::prelude::BootServices;
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;
use crate::utils::info::mem::MemInfo;
use crate::utils::protocols::{get_resolution, stdout_text_color};

pub(crate) fn draw(mut stdout: &mut ScopedProtocol<Output>, boot_services: &BootServices) {
    stdout.clear().unwrap();

    let mem = MemInfo::get(boot_services);
    let columns = get_resolution(&mut stdout).1;

    stdout_text_color(&mut stdout, Color::LightRed);
    println!("{:^width$}", "Mem page", width = columns);
    stdout_text_color(&mut stdout, Color::LightGray);
    print!("{:^width$}", format!("Total pages: {}", mem.total_pages), width = columns);
    print!("{:^width$}", format!("Used pages: {}", mem.used_pages), width = columns);
    
}
use alloc::format;

use uefi::{print, println};
use uefi::prelude::BootServices;
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;
use crate::utils::info::mem::MappedMemoryInfo;
use crate::utils::protocols::{get_resolution, stdout_text_color};

pub(crate) fn draw(mut stdout: &mut ScopedProtocol<Output>, boot_services: &BootServices) {
    stdout.clear().unwrap();

    let mem = MappedMemoryInfo::from(boot_services);
    let columns = get_resolution(&mut stdout).1;

    stdout_text_color(&mut stdout, Color::LightRed);
    println!("{:^width$}", "Mem page", width = columns);
    stdout_text_color(&mut stdout, Color::LightGray);
    print!("{:^width$}", format!("Total pages: {}", mem.info.pages.total), width = columns);
    print!("{:^width$}", format!("Used pages: {}", mem.info.pages.used), width = columns);
    print!("{:^width$}", format!("Physical start: {}", mem.info.phys_addr), width = columns);
    print!("{:^width$}", format!("Virtual start: {}", mem.info.virt_addr), width = columns);
    
}
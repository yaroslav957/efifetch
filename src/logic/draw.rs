use uefi::prelude::RuntimeServices;
use uefi::println;
use alloc::format;
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;
use crate::utils::info::{CpuInfo, Date, print_info};
use crate::utils::protocols::{get_resolution, stdout_text_color};

const LABEL: &str = "Efifetch 0.1.0";

pub fn draw_fetch(mut stdout: ScopedProtocol<Output>, runtime_services: &RuntimeServices) {
    stdout.clear().unwrap();

    let (rows, columns) = get_resolution(&mut stdout);
    let date = Date::get(runtime_services);
    let cpu = CpuInfo::get();

    stdout_text_color(&mut stdout, Color::LightRed);
    println!("{:^width$}", LABEL, width = columns);
    stdout_text_color(&mut stdout, Color::LightGray);
    print_info(format!("Resolution: {} x {}", columns, rows), columns);
    print_info(format!("Date: {:?}/{:?}/{:?}", date.day, date.month, date.year), columns);
    print_info(format!("Cpu brand: {:?}, Cpu Vendor: {:?}", cpu.brand.as_str(), cpu.vendor.as_str()), columns);
}



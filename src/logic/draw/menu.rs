use uefi::prelude::RuntimeServices;
use uefi::{print, println};
use alloc::format;
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;
use crate::utils::info::date::Date;
use crate::utils::protocols::{get_resolution, stdout_text_color};

pub(crate) fn draw(mut stdout: &mut ScopedProtocol<Output>, runtime_services: &RuntimeServices) {
    stdout.clear().unwrap();

    let date = Date::get(runtime_services);
    let (rows, columns) = get_resolution(&mut stdout);
    
    stdout_text_color(&mut stdout, Color::LightRed);
    println!("{:^width$}", "Efifetch 0.1.0", width = columns);
    stdout_text_color(&mut stdout, Color::LightGray);
    print!("{:^width$}", format!("Resolution: {} x {}", columns, rows), width = columns);
    print!("{:^width$}", format!("Bios Date: {}/{}/{}", date.day, date.month, date.year), width = columns);
    print!("{:^width$}", "Mem/RAM: press 1", width = columns);
    println!("{:^width$}", "Cpu: press 2", width = columns);
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("{:>width$}", "██", width = columns / 2 - 6);

    let colors = [
        Color::LightGreen,
        Color::LightBlue,
        Color::LightGray,
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::DarkGray];
    
    for color in colors {
        stdout_text_color(&mut stdout, color);
        print!("██");
    }
}



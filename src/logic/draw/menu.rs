use uefi::prelude::RuntimeServices;
use uefi::{print, println};
use alloc::{format, vec};
use alloc::string;
use alloc::string::{String, ToString};
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;
use crate::utils::info::Date;
use crate::utils::protocols::{get_resolution, stdout_text_color};

pub fn draw(mut stdout: &mut ScopedProtocol<Output>, runtime_services: &RuntimeServices) {
    stdout.clear().unwrap();

    let (rows, columns) = get_resolution(stdout);
    let date = Date::get(runtime_services);

    stdout_text_color(&mut stdout, Color::LightRed);
    println!("{:^width$}", "Efifetch 0.1.0", width = columns);
    stdout_text_color(&mut stdout, Color::LightGray);
    print!("{:^width$}", format!("Resolution: {} x {}", columns, rows), width = columns);
    println!("{:^width$}", format!("Bios Date: {}/{}/{}", date.day, date.month, date.year), width = columns);
    //
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("{:>width$}", "██", width = columns / 2 - 6);
    
    let colors = [Color::Red, Color::LightGreen, Color::Green, Color::LightBlue, Color::Blue, Color::LightGray, Color::DarkGray];
    for color in colors {
        stdout_text_color(&mut stdout, color);
        print!("██");
    }
}



use alloc::{format, vec};
use uefi::{print, println, Status};
use uefi::proto::console::text::{Color, Output, OutputMode};
use uefi::table::boot::ScopedProtocol;
use crate::utils::protocols::{change_text_color, get_resolution};

const LABEL: &str = "Efifetch 0.1.0";

pub fn main_loop(mut stdout: ScopedProtocol<Output>) -> Status {
    stdout.clear()
        .expect("Cant clear terminal");

    let (rows, columns) = get_resolution(&mut stdout);
    
    change_text_color(&mut stdout, Color::LightRed);
    println!("{:^width$}", LABEL, width = columns);
    
    
    let resolution = format!("Resolution: {}*{}", columns, rows);
    let args = vec![resolution];

    change_text_color(&mut stdout, Color::LightGray);
    
    for arg in args {
        print!("{:^width$}", arg, width = columns);
    }

    loop {}

    Status::SUCCESS
}
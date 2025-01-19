use alloc::format;

use crate::utils::info::mem::MappedMemoryInfo;
use crate::utils::protocols::{get_resolution, stdout_text_color};
use uefi::prelude::BootServices;
use uefi::print;
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;

pub(crate) fn draw(mut stdout: &mut ScopedProtocol<Output>, boot_services: &BootServices) {
    let mem = MappedMemoryInfo::from(boot_services);
    let columns = get_resolution(&mut stdout).1;

    stdout_text_color(&mut stdout, Color::LightRed);
    print!("{:<width$}", " Mem:", width = columns - 13);
    print!("██║        ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Total pages: {}", mem.info.pages.total),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("╚═╝        ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Used pages: {}", mem.info.pages.used),
        width = columns - 2
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Physical start: {}", mem.info.phys_addr),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("██╗        ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Virtual start: {}", mem.info.virt_addr),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("██║        ");
    print!("│");
}

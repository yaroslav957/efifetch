use alloc::format;

use crate::utils::info::cpu::CpuInfo;
use crate::utils::protocols::{get_resolution, stdout_text_color};
use uefi::print;
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;

pub(crate) fn draw(mut stdout: &mut ScopedProtocol<Output>) {
    let cpu = CpuInfo::get();
    let columns = get_resolution(&mut stdout).1;

    stdout_text_color(&mut stdout, Color::LightRed);
    print!("{:<width$}", " Cpu:", width = columns - 13);
    print!("╚══════╝   ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Brand: {}", cpu.brand.as_str()),
        width = columns - 2
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Vmx(x86 virtualization): {}", cpu.vmx),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("███████╗   ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Vendor: {}", cpu.vendor.as_str()),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("██╔════╝   ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Hypervisor: {}", cpu.hypervisor),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("█████╗     ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Smx: {:?}", cpu.smx),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("██╔══╝     ");
    print!("│");
    print!("│");
}

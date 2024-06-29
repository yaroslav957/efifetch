use alloc::format;
use uefi::{print, println};
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;
use crate::utils::info::CpuInfo;
use crate::utils::protocols::{get_resolution, stdout_text_color};

pub(crate) fn draw(mut stdout: &mut ScopedProtocol<Output>) {
    stdout.clear().unwrap();

    let cpu = CpuInfo::get();
    let columns = get_resolution(&mut stdout).1;

    stdout_text_color(&mut stdout, Color::LightRed);
    println!("{:^width$}", "Cpu page", width = columns);
    stdout_text_color(&mut stdout, Color::LightGray);
    print!("{:^width$}", format!("Brand: {}", cpu.brand.as_str()), width = columns);
    print!("{:^width$}", format!("Vmx (x86 virtualization): {}", cpu.vmx), width = columns);
    print!("{:^width$}", format!("Vendor: {}", cpu.vendor.as_str()), width = columns);
    print!("{:^width$}", format!("Hypervisor: {:?}", cpu.hypervisor.identify()), width = columns);
    print!("{:^width$}", format!("Smx: {:?}", cpu.smx), width = columns);
}
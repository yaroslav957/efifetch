use core::fmt::Write;

use crate::{
    logic::info::date::Date,
    utils::{get_resolution, stdout_text_color},
};
use alloc::format;
use uefi::{
    boot::ScopedProtocol, proto::console::text::{Color, Output}, Result
};

const LOGO: &'static str = include_str!("../../assets/uefi.logo");
const VERSION: &'static str = " Efifetch 0.1.9";
const COLORS: [Color; 14] = [
    Color::LightRed,
    Color::LightGreen,
    Color::LightBlue,
    Color::LightGray,
    Color::LightCyan,
    Color::LightMagenta,
    Color::White,
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::DarkGray,
    Color::Cyan,
    Color::Magenta,
    Color::Black,
];

pub fn draw(mut stdout: &mut ScopedProtocol<Output>, date: Date) -> Result<()> {
    //stdout.clear()?;

    let (rows, columns) = get_resolution(&mut stdout)?;
    let mut logo = LOGO.lines();
    

    let resolution = format!(" Resolution: {}x{}", columns, rows);
    let date = format!(" BIOS Date: {}/{}/{}", date.day, date.month, date.year);
    let revision = format!(" UEFI Revision: {}", uefi::system::uefi_revision());
    let firmware_vendor = format!(" Firmware Vendor: {}", uefi::system::firmware_vendor());
    let firmware_revision = format!(" Firmware Revision: {}", uefi::system::firmware_revision());

    // Top frame - 1 line
    let mut margin = columns - 35;
    stdout_text_color(&mut stdout, Color::LightRed)?;
    stdout
        .write_fmt(format_args!("┌{:─<32}┬{:─>margin$}┐", "", ""))
        .unwrap();

    // Info bar - 2 line
    if columns % 5 != 0 {
        margin = columns - 82;
    } else {
        margin = (columns - 80) / 5;
    }
    stdout
        .write_fmt(format_args!("│ {} │", logo.next().unwrap()))
        .unwrap();
    stdout_text_color(&mut stdout, Color::Red)?;
    stdout.write_str(" PCI:").unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;
    stdout.write_str("F1 ").unwrap();
    stdout_text_color(&mut stdout, Color::Red)?;
    stdout
        .write_fmt(format_args!("{:<margin$}CPU:", ""))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;
    stdout.write_str("F2 ").unwrap();
    stdout_text_color(&mut stdout, Color::Red)?;
    stdout
        .write_fmt(format_args!("{:<margin$}MEM:", ""))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;
    stdout.write_str("F3 ").unwrap();
    stdout_text_color(&mut stdout, Color::Red)?;
    stdout
        .write_fmt(format_args!("{:<margin$}NET:", ""))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;
    stdout.write_str("F4 ").unwrap();
    stdout_text_color(&mut stdout, Color::Red)?;
    stdout
        .write_fmt(format_args!("{:<margin$}ACPI:", ""))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;
    stdout.write_str("F5 ").unwrap();
    stdout_text_color(&mut stdout, Color::Red)?;
    stdout
        .write_fmt(format_args!("{:<margin$}HOST:", ""))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;
    stdout.write_str("F6 │").unwrap();

    // Underbar - 3 line
    let margin = columns - 35;
    stdout
        .write_fmt(format_args!(
            "│ {} ├{:─<margin$}┤",
            logo.next().unwrap(),
            ""
        ))
        .unwrap();

    // UEFI and runtime information - 4-9 lines
    // 4
    stdout
        .write_fmt(format_args!("│ {} │", logo.next().unwrap()))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightGray)?;
    stdout.write_fmt(format_args!("{VERSION}")).unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;

    let margin = columns - VERSION.len() - 35;
    stdout.write_fmt(format_args!("{:<margin$}│", "")).unwrap();

    // 5
    stdout
        .write_fmt(format_args!("│ {} │", logo.next().unwrap()))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightGray)?;
    stdout.write_fmt(format_args!("{resolution}")).unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;

    let margin = columns - resolution.len() - 35;
    stdout.write_fmt(format_args!("{:<margin$}│", "")).unwrap();

    // 6
    stdout
        .write_fmt(format_args!("│ {} │", logo.next().unwrap()))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightGray)?;
    stdout.write_fmt(format_args!("{date}")).unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;

    let margin = columns - date.len() - 35;
    stdout.write_fmt(format_args!("{:<margin$}│", "")).unwrap();

    // 7
    stdout
        .write_fmt(format_args!("│ {} │", logo.next().unwrap()))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightGray)?;
    stdout.write_fmt(format_args!("{revision}")).unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;

    let margin = columns - revision.len() - 35;
    stdout.write_fmt(format_args!("{:<margin$}│", "")).unwrap();

    // 8
    stdout
        .write_fmt(format_args!("│ {} │", logo.next().unwrap()))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightGray)?;
    stdout
        .write_fmt(format_args!("{firmware_revision}"))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;

    let margin = columns - firmware_revision.len() - 35;
    stdout.write_fmt(format_args!("{:<margin$}│", "")).unwrap();

    // 9
    stdout
        .write_fmt(format_args!("│ {} │", logo.next().unwrap()))
        .unwrap();
    stdout_text_color(&mut stdout, Color::LightGray)?;
    stdout.write_fmt(format_args!("{firmware_vendor}")).unwrap();
    stdout_text_color(&mut stdout, Color::LightRed)?;

    let margin = columns - firmware_vendor.len() - 35;
    stdout.write_fmt(format_args!("{:<margin$}│", "")).unwrap();

    // Blank lines
    let margin = columns - 35;
    stdout
        .write_fmt(format_args!("│ {} │{:<margin$}│", logo.next().unwrap(), ""))
        .unwrap();
    stdout
        .write_fmt(format_args!("│ {} │{:<margin$}│", logo.next().unwrap(), ""))
        .unwrap();
    stdout
        .write_fmt(format_args!("│ {} │{:<margin$}│", logo.next().unwrap(), ""))
        .unwrap();
    stdout
        .write_fmt(format_args!("│ {} │{:<margin$}│", logo.next().unwrap(), ""))
        .unwrap();
    stdout
        .write_fmt(format_args!("│ {} │{:<margin$}│", logo.next().unwrap(), ""))
        .unwrap();
    stdout
        .write_fmt(format_args!("│ {} │{:<margin$}│", logo.next().unwrap(), ""))
        .unwrap();
    stdout
        .write_fmt(format_args!("│ {} │{:<margin$}│", logo.next().unwrap(), ""))
        .unwrap();
    stdout
        .write_fmt(format_args!("│ {} │{:<margin$}│", logo.next().unwrap(), ""))
        .unwrap();

    stdout
        .write_fmt(format_args!("│{:<32}│{:>margin$}││ ", "", ""))
        .unwrap();

    for i in 0..=6 {
        stdout_text_color(&mut stdout, COLORS[i])?;
        stdout.write_str("██").unwrap();
    }

    stdout_text_color(&mut stdout, Color::LightRed)?;

    let margin = columns - 35;
    stdout
        .write_fmt(format_args!("{:<17}│{:<margin$}││ ", "", ""))
        .unwrap();

    for i in 7..=13 {
        stdout_text_color(&mut stdout, COLORS[i])?;
        stdout.write_str("██").unwrap();
    }

    stdout_text_color(&mut stdout, Color::LightRed)?;

    let margin = columns - 35;
    stdout
        .write_fmt(format_args!("{:<17}│{:<margin$}│", "", ""))
        .unwrap();

    // Bottom bar - 21 line
    stdout
        .write_fmt(format_args!("└{:─<32}┴{:─<margin$}┘", "", ""))
        .unwrap();
    Ok(())
}

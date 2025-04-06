use crate::{
    logic::info::date::Date,
    utils::protocols::{get_resolution, stdout_text_color},
};
use alloc::format;
use uefi::{
    boot::ScopedProtocol,
    print,
    proto::console::text::{Color, Output},
    Result,
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
    stdout.clear()?;

    let (rows, columns) = get_resolution(&mut stdout)?;
    let mut logo = LOGO.lines();

    let resolution = format!(" Resolution: {}x{}", columns, rows);
    let date = format!(" BIOS Date: {}/{}/{}", date.day, date.month, date.year);
    let revision = format!(" UEFI Revision: {}", uefi::system::uefi_revision());
    let firmware_vendor = format!(" Firmware Vendor: {}", uefi::system::firmware_vendor());
    let firmware_revision = format!(" Firmware Revision: {}", uefi::system::firmware_revision());

    // Top frame - 1 line
    //==================================================================//
    let margin = columns - 35;
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("┌{:─<32}┬{:─>margin$}┐", "", "");
    //==================================================================//

    // Info bar - 2 line
    //==================================================================//
    let margin = (columns - 80) / 5;
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::Red)?;
    print!(" NET:");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F1 ");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<margin$}CPU:", "");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F2 ");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<margin$}MEM:", "");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F3 ");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<margin$}PCI:", "");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F4 ");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<margin$}ACPI:", "");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F5 ");
    stdout_text_color(&mut stdout, Color::Red)?;
    print!("{:<margin$}HOST:", "");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("F6 │");
    //==================================================================//

    // Underbar - 3 line
    //==================================================================//
    let margin = columns - 35;
    print!("│ {} ├{:─<margin$}┤", logo.next().unwrap(), "");
    //==================================================================//
    
    // UEFI and runtime information - 4-9 lines 
    //==================================================================//
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{VERSION}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}│", "", width = columns - VERSION.len() - 35);
    // 5
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{resolution}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}│", "", width = columns - resolution.len() - 35);
    // 6
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{date}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}│", "", width = columns - date.len() - 35);
    // 7
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{revision}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!("{:<width$}│", "", width = columns - revision.len() - 35);
    // 8
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{firmware_revision}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<width$}│",
        "",
        width = columns - firmware_revision.len() - 35
    );
    // 9
    print!("│ {} │", logo.next().unwrap());
    stdout_text_color(&mut stdout, Color::LightGray)?;
    print!("{firmware_vendor}");
    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<width$}│",
        "",
        width = columns - firmware_vendor.len() - 35
    );
    //==================================================================//
    
    // Blank lines
    //==================================================================//
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!(
        "│ {} │{:<width$}│",
        logo.next().unwrap(),
        "",
        width = columns - 35
    );
    print!("│{:<32}│{:>width$}││ ", "", "", width = columns - 35);

    for i in 0..=6 {
        stdout_text_color(&mut stdout, COLORS[i])?;
        print!("██");
    }

    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<left_space$}│{:<right_space$}││ ",
        "",
        "",
        left_space = 17,
        right_space = columns - 35
    );

    for i in 7..=13 {
        stdout_text_color(&mut stdout, COLORS[i])?;
        print!("██");
    }

    stdout_text_color(&mut stdout, Color::LightRed)?;
    print!(
        "{:<left_space$}│{:<right_space$}│",
        "",
        "",
        left_space = 17,
        right_space = columns - 35
    );
    //==================================================================//
    
    // Bottom bar - 21 line
    //==================================================================//
    print!(
        "└{:─<32}┴{:─<right_space$}┘",
        "",
        "",
        right_space = columns - 35
    );
    //==================================================================//
    Ok(())
}

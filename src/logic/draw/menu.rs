use crate::logic::draw::cpu;
use crate::logic::mem;
use crate::utils::info::date::Date;
use crate::utils::protocols::{get_resolution, stdout_text_color};
use alloc::format;
use uefi::prelude::{BootServices, RuntimeServices};
use uefi::print;
use uefi::proto::console::text::{Color, Output};
use uefi::table::boot::ScopedProtocol;

const VERSION: &'static str = " Efifetch 0.1.8 ";

pub(crate) fn draw(
    mut stdout: &mut ScopedProtocol<Output>,
    runtime_services: &RuntimeServices,
    boot_services: &BootServices,
) {
    stdout.clear().unwrap();

    let date = Date::get(runtime_services);
    let (rows, columns) = get_resolution(&mut stdout);
    let colors = [
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

    stdout_text_color(&mut stdout, Color::LightRed);
    print!(
        "┌{:─<left_space$}{}{:─<right_space$}x─┐",
        "",
        VERSION,
        "",
        left_space = (columns - VERSION.chars().count() - 4) / 2,
        right_space = (columns - VERSION.chars().count() - 4 + 1) / 2
    );
    print!("│");
    print!("{:<width$}", format!(" Utility: "), width = columns - 13);
    print!("███████╗   ");
    print!("│");
    print!("│");

    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Resolution: {} x {}", columns, rows),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("██╔════╝   ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, Color::LightGray);
    print!(
        "{:<width$}",
        format!(" Bios Date: {}/{}/{}", date.day, date.month, date.year),
        width = columns - 13
    );
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("█████╗     ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, colors[0]);
    print!(" ██");
    stdout_text_color(&mut stdout, colors[1]);
    print!("██");
    stdout_text_color(&mut stdout, colors[2]);
    print!("██");
    stdout_text_color(&mut stdout, colors[3]);
    print!("██");
    stdout_text_color(&mut stdout, colors[4]);
    print!("██");
    stdout_text_color(&mut stdout, colors[5]);
    print!("██");
    stdout_text_color(&mut stdout, colors[6]);
    print!("██");
    print!("{:<width$}", format!(" "), width = columns - 28);
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("██╔══╝     ");
    print!("│");
    print!("│");
    stdout_text_color(&mut stdout, colors[7]);
    print!(" ██");
    stdout_text_color(&mut stdout, colors[8]);
    print!("██");
    stdout_text_color(&mut stdout, colors[9]);
    print!("██");
    stdout_text_color(&mut stdout, colors[10]);
    print!("██");
    stdout_text_color(&mut stdout, colors[11]);
    print!("██");
    stdout_text_color(&mut stdout, colors[12]);
    print!("██");
    stdout_text_color(&mut stdout, colors[13]);
    print!("██");
    print!("{:<width$}", format!(" "), width = columns - 17 - 11);
    stdout_text_color(&mut stdout, Color::LightRed);
    print!("███████╗   ");
    print!("│");
    print!("│");
    cpu::draw(&mut stdout);
    mem::draw(&mut stdout, boot_services);
    print!("│");
    print!("{:<width$}", format!(" "), width = columns - 13);
    print!("██║        ");
    print!("│");
    print!("│");
    print!("{:<width$}", format!(" "), width = columns - 13);
    print!("██║        ");
    print!("│");
    print!("│");
    print!("{:<width$}", format!(" "), width = columns - 13);
    print!("██║        ");
    print!("│");
    print!("│");
    print!("{:<width$}", format!(" "), width = columns - 13);
    print!("╚═╝        ");
    print!("│");
    print!(
        "└{:─<left_space$}{:─<right_space$}┘",
        "",
        "",
        left_space = (columns - 2) / 2,
        right_space = (columns - 2 + 1) / 2
    );
}

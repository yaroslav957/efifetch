use crate::{Flags, error::Result, info::Info, output::page::Page};

use alloc::vec::Vec;
use core::{cmp::max, fmt::Write};

use uefi::{
    boot::ScopedProtocol,
    proto::console::text::{Color, Output},
};

pub mod page;
pub mod theme;

const LOGO_LINES: usize = 16;
const INFO_START: usize = 1;
const LOGO: &str = include_str!("./../assets/uefi.logo");
const PALETTE: [Color; 16] = [
    Color::Black,
    Color::Blue,
    Color::Green,
    Color::Cyan,
    Color::Red,
    Color::Magenta,
    Color::Brown,
    Color::LightGray,
    Color::DarkGray,
    Color::LightBlue,
    Color::LightGreen,
    Color::LightCyan,
    Color::LightRed,
    Color::LightMagenta,
    Color::Yellow,
    Color::White,
];

pub fn draw(
    stdout: &mut ScopedProtocol<Output>,
    info: Info,
    flags: Flags,
) -> Result<()> {
    let mut rows: Vec<(&str, &str)> = Vec::new();

    match flags.page {
        Page::Main => Page::Main.add(&mut rows, &info)?,
        Page::Firmware => Page::Firmware.add(&mut rows, &info)?,
        Page::Memory => Page::Memory.add(&mut rows, &info)?,
    }

    let total_lines = max(LOGO_LINES, rows.len());
    let color_row = if total_lines > 2 { total_lines - 2 } else { 0 };
    let mut logo = LOGO.lines();

    for i in 0..total_lines {
        stdout.set_color(
            flags.theme.logo.foreground,
            flags.theme.logo.background,
        )?;

        if let Some(line) = logo.next() {
            write!(stdout, "{line}")?;

            let len = line.len();
            if len < INFO_START {
                write!(stdout, "{:width$}", "", width = INFO_START - len)?;
            }
        } else {
            write!(stdout, "{:width$}", "", width = INFO_START)?;
        }

        if let Some((label, value)) = rows.get(i) {
            stdout.set_color(
                flags.theme.label.foreground,
                flags.theme.label.background,
            )?;
            write!(stdout, "{label} ")?;
            stdout.set_color(
                flags.theme.content.foreground,
                flags.theme.content.background,
            )?;
            write!(stdout, "{value}")?;
        } else if i >= color_row {
            let idx = i - color_row;
            if let Some(chunk) = PALETTE.chunks(8).nth(idx) {
                for &color in chunk {
                    stdout.set_color(color, Color::Black)?;
                    write!(stdout, "██")?;
                }
            }
        }

        writeln!(stdout, "")?;
    }

    stdout.set_color(Color::LightGray, Color::Black)?;
    Ok(())
}

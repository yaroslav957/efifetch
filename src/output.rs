use crate::{
    error::Result,
    info::Info,
    output::{page::Page, theme::Theme},
};

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
    page: Page,
    theme: Theme,
) -> Result<()> {
    let mut rows = Vec::new();

    page.add(&mut rows, &info)?;

    let total_lines = max(LOGO_LINES, rows.len());
    let palette_start = total_lines.saturating_sub(2);
    let mut logo = LOGO.lines().fuse();

    for i in 0..total_lines {
        stdout.set_color(theme.logo.foreground, theme.logo.background)?;

        if let Some(line) = logo.next() {
            write!(stdout, "{line:width$}", width = INFO_START)?;
        }

        if let Some((label, value)) = rows.get(i) {
            stdout.set_color(theme.label.foreground, theme.label.background)?;
            write!(stdout, "{label} ")?;
            stdout.set_color(
                theme.content.foreground,
                theme.content.background,
            )?;
            write!(stdout, "{value}")?;
        } else if i >= palette_start {
            let idx = i - palette_start;
            if let Some(colors) = PALETTE.chunks(8).nth(idx) {
                for &color in colors {
                    stdout.set_color(color, Color::Black)?;
                    write!(stdout, "███")?;
                }
            }
        }

        writeln!(stdout)?;
    }

    stdout.set_color(Color::LightGray, Color::Black)?;

    Ok(())
}

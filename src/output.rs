pub mod page;
pub mod theme;

use crate::{Flags, error::Result, info::Info, output::page::Page};
use core::{cmp::max, fmt::Write};
use heapless::Vec;
use uefi::{
    boot::ScopedProtocol,
    proto::console::text::{Color, Output},
};

const LOGO_LINES: usize = 16;
const INFO_START: usize = 1;

pub fn draw(
    stdout: &mut ScopedProtocol<Output>,
    info: Info,
    flags: Flags,
) -> Result<()> {
    let mut rows: Vec<(&str, &str), 32> = Vec::new();

    match flags.page {
        Page::Main => Page::Main.add(&mut rows, &info)?,
        Page::Env => Page::Env.add(&mut rows, &info)?,
    }

    let mut logo = info.env.logo.lines();
    let total_lines = if flags.logo {
        max(LOGO_LINES, rows.len())
    } else {
        rows.len()
    };

    for i in 0..total_lines {
        if flags.logo {
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
            } else if i < LOGO_LINES || i < rows.len() {
                write!(stdout, "{:width$}", "", width = INFO_START)?;
            }
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
        }
        writeln!(stdout, "")?;
    }

    stdout.set_color(Color::LightGray, Color::Black)?;

    Ok(())
}

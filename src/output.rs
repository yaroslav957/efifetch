pub mod page;
pub mod theme;

use crate::{
    Flags,
    error::Result,
    info::{Info, InfoItem},
    output::theme::Theme,
};
use core::{cmp::max, fmt::Write};
use heapless::Vec;
use uefi::{
    boot::ScopedProtocol,
    proto::console::text::{Color, Output},
};

const LOGO_LINES: usize = 15;
const INFO_START: usize = 1;

pub fn draw(
    stdout: &mut ScopedProtocol<Output>,
    info: Info,
    theme: Theme,
    flags: Flags,
) -> Result<()> {
    let mut rows: Vec<(&str, &str), 32> = Vec::new();

    // TODO:
    // use crate::output::page::Page;
    //
    // match flags.page {
    //     Page::MAIN => rows = Page::main(),
    //     Page::CPU => rows = Page::cpu(),
    //     ... => ...,
    //     _ => Page::default(),
    // };
    //
    // TODO: move inside Page enum
    add_to_rows(&mut rows, &info.date);
    add_to_rows(&mut rows, &info.firmware);
    add_to_rows(&mut rows, &info.env);

    let mut logo = info.env.logo.lines();
    let total_lines = if flags.logo {
        max(LOGO_LINES, rows.len())
    } else {
        rows.len()
    };

    for i in 0..total_lines {
        if flags.logo {
            stdout.set_color(theme.logo.foreground, theme.logo.background)?;

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
            stdout.set_color(theme.label.foreground, theme.label.background)?;
            write!(stdout, "{label} ")?;
            stdout.set_color(
                theme.content.foreground,
                theme.content.background,
            )?;
            write!(stdout, "{value}")?;
        }
        writeln!(stdout, "")?;
    }

    // Setting default UEFI-Shell color
    // for CLI before exit draw fn
    stdout.set_color(Color::LightGray, Color::Black)?;

    Ok(())
}

fn add_to_rows<'r, T, const N: usize>(
    rows: &mut Vec<(&'r str, &'r str), N>,
    item: &'r T,
) where
    T: InfoItem,
{
    for row in item.render() {
        _ = rows.push(row);
    }
}

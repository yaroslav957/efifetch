use crate::{
    Out, cursor,
    display::{Page, Theme, topbar},
    draw,
};
use core::fmt::Write;
use uefi::Result;

pub fn draw(out: &mut Out, width: usize, height: usize, theme: Theme) -> Result<()> {
    let width = width - 2;
    let height = height - 4;

    cursor!(out, 0, 1);
    header(out, width, theme);
    label(out, width, height, theme)?;
    footer(out, width, theme);

    topbar::update(out, theme, Page::About)?;

    Ok(())
}

fn header(out: &mut Out, width: usize, theme: Theme) {
    draw!(out, theme.page.fg, theme.page.bg, "┌{:─<width$}┐", "");
}

fn label(out: &mut Out, width: usize, height: usize, theme: Theme) -> Result<()> {
    (0..height).for_each(|_| {
        draw!(out, theme.page.fg, theme.page.bg, "│{:<width$}│", "");
    });

    Ok(())
}

fn footer(out: &mut Out, width: usize, theme: Theme) {
    draw!(out, theme.page.fg, theme.page.bg, "└{:─<width$}┘", "");
}

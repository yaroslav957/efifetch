use crate::{
    Out, cursor,
    display::{Theme, page::Page},
    draw,
};
use core::fmt::Write;
use uefi::Result;

const PAGES: &[&str] = &["Main", "About", "Exit"];

pub fn draw(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    let margin = width - PAGES.iter().map(|p| p.len() + 2).sum::<usize>();

    draw_pages(out, theme, PAGES)?;
    draw!(out, theme.topbar_fg, theme.topbar_bg, "{:<margin$}", "");

    Ok(())
}

pub fn update(out: &mut Out, theme: Theme, page: Page) -> Result<()> {
    cursor!(out, 0, 0);
    draw_pages(out, theme, PAGES)?;

    match page {
        Page::Main => {
            cursor!(out, 0, 0);
            draw!(out, theme.page_fg, theme.page_bg, " Main ");

            Ok(())
        }

        Page::About => {
            cursor!(out, 6, 0);
            draw!(out, theme.page_fg, theme.page_bg, " About ");

            Ok(())
        }

        Page::Exit => {
            cursor!(out, 13, 0);
            draw!(out, theme.page_fg, theme.page_bg, " Exit ");

            Ok(())
        }
    }
}

fn draw_pages(out: &mut Out, theme: Theme, pages: &[&str]) -> Result<()> {
    pages.iter().try_for_each(|p| {
        draw!(out, theme.highlight_fg, theme.topbar_bg, " {}", &p[0..1]);
        draw!(out, theme.topbar_fg, theme.topbar_bg, "{} ", &p[1..]);

        Ok(())
    })
}

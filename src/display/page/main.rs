use crate::{
    Out,
    display::{Theme, page::Page, topbar},
    draw,
};
use core::fmt::Write;
use uefi::Result;

pub fn draw(out: &mut Out, width: usize, height: usize, theme: Theme) -> Result<()> {
    let screen_height = height - 5;
    let margin = (width - "<switch to page>".len()) / 2;

    (0..screen_height).for_each(|i| {
        if i == screen_height / 2 {
            draw!(
                out,
                theme.page.0,
                theme.page.1,
                "{:<margin$}<switch to page>{:<margin$}",
                "",
                ""
            );
        }

        draw!(out, theme.page.0, theme.page.1, "{:<width$}", "");
    });

    topbar::update(out, theme, Page::Main)?;

    uefi::boot::stall(2_000_000);

    topbar::update(out, theme, Page::About)?;

    uefi::boot::stall(2_000_000);

    topbar::update(out, theme, Page::Exit)?;

    Ok(())
}

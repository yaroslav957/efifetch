use crate::{
    Out,
    display::{Theme, page::Page, topbar::update},
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
                theme.page_fg,
                theme.page_bg,
                "{:<margin$}<switch to page>{:<margin$}",
                "",
                ""
            );
        }

        draw!(out, theme.page_fg, theme.page_bg, "{:<width$}", "");
    });

    update(out, theme, Page::Main)?;

    Ok(())
}

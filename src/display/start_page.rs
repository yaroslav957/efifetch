use crate::{Out, display::Theme};
use core::fmt::Write;
use uefi::Result;

pub fn draw(out: &mut Out, width: usize, height: usize, theme: Theme) -> Result<()> {
    let screen_height = height - 5;
    let margin = (width - "<switch to page>".len()) / 2;

    color!(out, theme.foreground, theme.background);

    (0..screen_height).try_for_each(|i| {
        if i == screen_height / 2 {
            draw!(out, "{:<margin$}<switch to page>{:<margin$}", "", "");
        }

        draw!(out, "{:<width$}", "");

        Ok(())
    })
}

use crate::{Out, display::Theme};
use core::fmt::Write;
use uefi::Result;

pub fn draw(out: &mut Out, width: usize, height: usize, theme: Theme) -> Result<()> {
    let screen_height = height - 5;
    let margin = (width - "<switch to page>".len()) / 2;

    (0..screen_height).try_for_each(|i| {
        if i == screen_height / 2 {
            draw!(
                out,
                theme.foreground,
                theme.background,
                "{:<margin$}<switch to page>{:<margin$}",
                "",
                ""
            );
        }

        draw!(out, theme.foreground, theme.background, "{:<width$}", "");

        Ok(())
    })
}

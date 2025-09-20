use crate::{Out, display::Theme};
use core::fmt::Write;
use uefi::Result;

pub fn draw(out: &mut Out, width: usize, height: usize, theme: Theme) -> Result<()> {
    out.set_color(theme.foreground, theme.background)?;

    let screen_height = height - 5;
    let margin = (width - "<switch to page>".len()) / 2;

    for i in 0..screen_height {
        if i == screen_height / 2 {
            out.write_fmt(format_args!(
                "{:<margin$}<switch to page>{:<margin$}",
                "", ""
            ))
            .unwrap()
        }
        out.write_fmt(format_args!("{:<width$}", "")).unwrap()
    }

    Ok(())
}

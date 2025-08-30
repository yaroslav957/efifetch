use core::fmt::Write;

use crate::{
    Out,
    display::{SCREEN_BACKGROUND, SCREEN_FOREGROUND},
};
use uefi::Result;

pub fn draw(out: &mut Out, width: usize, height: usize) -> Result<()> {
    out.set_color(SCREEN_FOREGROUND, SCREEN_BACKGROUND)?;

    let screen_height = height - 3;
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

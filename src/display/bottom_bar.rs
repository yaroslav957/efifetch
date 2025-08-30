use crate::{
    Out,
    display::{PANEL_BACKGROUND, PANEL_FOREGROUND},
};
use core::fmt::Write;
use uefi::Result;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub fn draw(out: &mut Out, width: usize) -> Result<()> {
    let margin = (width - NAME.len() - VERSION.len() - AUTHOR.len() - 6) / 2;
    out.set_color(PANEL_FOREGROUND, PANEL_BACKGROUND)?;
    out.write_fmt(format_args!("{:<margin$}{NAME} {VERSION}, by {AUTHOR}", ""))
        .unwrap();

    Ok(())
}

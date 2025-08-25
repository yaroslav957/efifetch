use crate::{
    Out,
    display::{PANELS_BACKGROUND, PANELS_FOREGROUND},
};
use core::fmt::Write;
use uefi::Result;

const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub fn draw(out: &mut Out, width: usize) -> Result<()> {
    let repository = &REPOSITORY[8..];
    let margin = (width - AUTHOR.len() - repository.len() - 6) / 2;

    out.set_color(PANELS_FOREGROUND, PANELS_BACKGROUND)?;
    out.write_fmt(format_args!(
        "{:<margin$}{AUTHOR}, see {repository}{:<margin$}",
        "", "",
    ))
    .unwrap();
    out.write_fmt(format_args!("{:<width$}", "")).unwrap();

    Ok(())
}

pub mod theme;

use crate::{
    consts::env::LOGO, error::Result, info::Info, output::theme::Theme,
};
use core::fmt::Write;
use uefi::{boot::ScopedProtocol, proto::console::text::Output};

#[derive(Clone, Copy, Default)]
pub enum Page {
    #[default]
    Main,
}

pub fn draw(
    stdout: &mut ScopedProtocol<Output>,
    info: Info,
    theme: Theme,
    logo: bool,
) -> Result<()> {
    if logo {
        draw_logo(stdout, theme)?;
    }

    stdout.set_color(theme.label.foreground, theme.label.background)?;
    // write!(stdout, "{:?}", info)?;

    Ok(())
}

fn draw_logo(stdout: &mut ScopedProtocol<Output>, theme: Theme) -> Result<()> {
    stdout.set_color(theme.logo.foreground, theme.logo.background)?;

    for line in LOGO.lines() {
        writeln!(stdout, "{line}")?
    }

    Ok(())
}

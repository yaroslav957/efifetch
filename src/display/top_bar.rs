use crate::{Out, display::Theme};
use core::fmt::Write;
use uefi::Result;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn draw(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    header(out, width, theme)?;
    label(out, width, theme)?;
    footer(out, width);

    Ok(())
}

fn header(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    let margin_left = NAME.len() + VERSION.len() + 3;
    let margin_right = width - margin_left - 3;

    out.set_color(theme.foreground, theme.background)?;
    out.write_fmt(format_args!(
        "┌{:─<margin_left$}┬{:─<margin_right$}┐",
        "", ""
    ))
    .unwrap();

    Ok(())
}

fn label(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    let pages = &["Cpu", "Mem", "Pci", "Net"];
    let bindings = &[("About", "F1"), ("Exit", "DEL")];
    let margin = width
        - pages.iter().map(|p| p.len() + 2).sum::<usize>()
        - bindings
            .iter()
            .map(|(label, key)| label.len() + 1 + key.len() + 1)
            .sum::<usize>()
        - 19;

    draw_package(out, theme)?;
    draw_pages(out, pages, theme)?;
    out.write_fmt(format_args!("{:<margin$}", "")).unwrap();
    draw_keybindings(out, bindings, theme)?;
    out.write_char('│').unwrap();

    Ok(())
}

fn draw_package(out: &mut Out, theme: Theme) -> Result<()> {
    out.write_char('│').unwrap();
    out.set_color(theme.highlight, theme.background)?;
    out.write_fmt(format_args!(" {NAME} {VERSION} ")).unwrap();
    out.set_color(theme.foreground, theme.background)?;
    out.write_char('│').unwrap();

    Ok(())
}

fn draw_keybindings(out: &mut Out, bindings: &[(&str, &str)], theme: Theme) -> Result<()> {
    for binding in bindings {
        out.set_color(theme.foreground, theme.background)?;
        out.write_fmt(format_args!("{}:", binding.0)).unwrap();
        out.set_color(theme.highlight, theme.background)?;
        out.write_fmt(format_args!("{} ", binding.1)).unwrap();
        out.set_color(theme.foreground, theme.background)?;
    }

    Ok(())
}

fn draw_pages(out: &mut Out, pages: &[&str], theme: Theme) -> Result<()> {
    for page in pages {
        out.set_color(theme.highlight, theme.background)?;
        out.write_char(' ').unwrap();
        out.write_str(&page[0..1]).unwrap();
        out.set_color(theme.foreground, theme.background)?;
        out.write_str(&page[1..]).unwrap();
        out.write_char(' ').unwrap();
    }

    Ok(())
}

fn footer(out: &mut Out, width: usize) {
    let margin_left = NAME.len() + VERSION.len() + 3;
    let margin_right = width - margin_left - 3;

    out.write_fmt(format_args!(
        "└{:─<margin_left$}┴{:─<margin_right$}┘",
        "", ""
    ))
    .unwrap();
}

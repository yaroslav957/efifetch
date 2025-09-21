use crate::{Out, display::Theme};
use core::fmt::Write;
use uefi::Result;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn draw(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    header(out, width, theme);
    label(out, width, theme)?;
    footer(out, width);

    Ok(())
}

fn header(out: &mut Out, width: usize, theme: Theme) {
    let symbols_count = "┌┬┐".chars().count();
    let margin_left = NAME.len() + VERSION.len() + symbols_count;
    let margin_right = width - margin_left - symbols_count;

    color!(out, theme.foreground, theme.background);
    draw!(out, "┌{:─<margin_left$}┬{:─<margin_right$}┐", "", "");
}

fn label(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    let pages = &["Cpu", "Mem", "Pci", "Net"];
    let bindings = &[("About", "F1"), ("Exit", "DEL")];
    let margin = width
        - pages.iter().map(|p| p.len() + 2).sum::<usize>()
        - bindings
            .iter()
            .map(|(label, key)| label.len() + key.len() + 2)
            .sum::<usize>()
        - 19;

    draw_package(out, theme);
    draw_pages(out, theme, pages)?;
    draw!(out, "{:<margin$}", "");
    draw_keybindings(out, theme, bindings)?;
    draw!(out, "│");

    Ok(())
}

fn draw_package(out: &mut Out, theme: Theme) {
    color!(out, theme.foreground, theme.background);
    draw!(out, "│");
    color!(out, theme.highlight, theme.background);
    draw!(out, " {NAME} {VERSION} ");
    color!(out, theme.foreground, theme.background);
    draw!(out, "│");
}

fn draw_pages(out: &mut Out, theme: Theme, pages: &[&str]) -> Result<()> {
    pages.iter().try_for_each(|p| {
        color!(out, theme.highlight, theme.background);
        draw!(out, " {}", &p[0..1]);
        color!(out, theme.foreground, theme.background);
        draw!(out, "{} ", &p[1..]);

        Ok(())
    })
}

fn draw_keybindings(out: &mut Out, theme: Theme, bindings: &[(&str, &str)]) -> Result<()> {
    bindings.iter().try_for_each(|b| {
        color!(out, theme.foreground, theme.background);
        draw!(out, "{}:", b.0);
        color!(out, theme.highlight, theme.background);
        draw!(out, "{} ", b.1);
        color!(out, theme.foreground, theme.background);

        Ok(())
    })
}

fn footer(out: &mut Out, width: usize) {
    let symbols_count = "└┴┘".chars().count();
    let margin_left = NAME.len() + VERSION.len() + symbols_count;
    let margin_right = width - margin_left - symbols_count;

    draw!(out, "└{:─<margin_left$}┴{:─<margin_right$}┘", "", "");
}

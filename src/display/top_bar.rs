use crate::{Out, display::Theme};
use core::fmt::Write;
use uefi::Result;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn draw(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    header(out, width, theme);
    label(out, width, theme)?;
    footer(out, width, theme);

    Ok(())
}

fn header(out: &mut Out, width: usize, theme: Theme) {
    let symbols_count = "┌┬┐".chars().count();
    let margin_left = NAME.len() + VERSION.len() + symbols_count;
    let margin_right = width - margin_left - symbols_count;

    draw!(
        out,
        theme.foreground,
        theme.background,
        "┌{:─<margin_left$}┬{:─<margin_right$}┐",
        "",
        ""
    );
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
    draw!(out, theme.foreground, theme.background, "{:<margin$}", "");
    draw_keybindings(out, theme, bindings)?;
    draw!(out, theme.foreground, theme.background, "│");

    Ok(())
}

fn draw_package(out: &mut Out, theme: Theme) {
    draw!(out, theme.foreground, theme.background, "│");
    draw!(out, theme.highlight, theme.background, " {NAME} {VERSION} ");
    draw!(out, theme.foreground, theme.background, "│");
}

fn draw_pages(out: &mut Out, theme: Theme, pages: &[&str]) -> Result<()> {
    pages.iter().try_for_each(|p| {
        draw!(out, theme.highlight, theme.background, " {}", &p[0..1]);
        draw!(out, theme.foreground, theme.background, "{} ", &p[1..]);

        Ok(())
    })
}

fn draw_keybindings(out: &mut Out, theme: Theme, bindings: &[(&str, &str)]) -> Result<()> {
    bindings.iter().try_for_each(|b| {
        draw!(out, theme.foreground, theme.background, "{}:", b.0);
        draw!(out, theme.highlight, theme.background, "{} ", b.1);

        Ok(())
    })
}

fn footer(out: &mut Out, width: usize, theme: Theme) {
    let symbols_count = "└┴┘".chars().count();
    let margin_left = NAME.len() + VERSION.len() + symbols_count;
    let margin_right = width - margin_left - symbols_count;

    draw!(
        out,
        theme.foreground,
        theme.background,
        "└{:─<margin_left$}┴{:─<margin_right$}┘",
        "",
        ""
    );
}

use crate::{
    Out,
    display::{BAR_BACKGROUND, BAR_FOREGROUND, BAR_HIGHLIGHT},
};
use core::fmt::Write;
use uefi::Result;

pub fn draw(out: &mut Out, width: usize) -> Result<()> {
    let pages = ["Cpu", "Mem", "Pci", "Net"];
    let bindings = [("About", "F1"), ("Exit", "DEL")];
    let margin = width
        - pages.iter().map(|p| p.len() + 2).sum::<usize>()
        - bindings
            .iter()
            .map(|(label, key)| label.len() + 1 + key.len() + 1)
            .sum::<usize>();

    draw_pages(out, &pages)?;
    out.write_fmt(format_args!("{:<margin$}", "")).unwrap();
    draw_keybindings(out, &bindings)?;

    Ok(())
}

fn draw_keybindings(out: &mut Out, bindings: &[(&str, &str)]) -> Result<()> {
    for binding in bindings {
        out.set_color(BAR_FOREGROUND, BAR_BACKGROUND)?;
        out.write_fmt(format_args!("{}:", binding.0)).unwrap();
        out.set_color(BAR_HIGHLIGHT, BAR_BACKGROUND)?;
        out.write_fmt(format_args!("{} ", binding.1)).unwrap();
    }

    Ok(())
}

fn draw_pages(out: &mut Out, pages: &[&str]) -> Result<()> {
    for page in pages {
        out.set_color(BAR_HIGHLIGHT, BAR_BACKGROUND)?;
        out.write_char(' ').unwrap();
        out.write_str(&page[0..1]).unwrap();
        out.set_color(BAR_FOREGROUND, BAR_BACKGROUND)?;
        out.write_str(&page[1..]).unwrap();
        out.write_char(' ').unwrap();
    }

    Ok(())
}

use crate::{Out, display::Theme, info::Info, utils::digits_count};
use core::fmt::Write;
use uefi::Result;

pub fn draw(out: &mut Out, width: usize, theme: Theme, info: &Info) -> Result<()> {
    cursor!(out, 0, 3);
    header(out, width, theme)?;
    label(out, width, theme, info)?;

    Ok(())
}

fn header(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    let align = width - "[Memory Info]".len();

    draw!(out, theme.highlight, theme.background, "[Memory Info]");
    draw!(out, theme.background, theme.background, "{:<align$}", "");

    Ok(())
}

fn label(out: &mut Out, width: usize, theme: Theme, info: &Info) -> Result<()> {
    let categories = &[
        ("└─Total RAM (mb): ", info.memory.total_memory),
        ("└─Usable RAM (mb): ", info.memory.usable_memory),
        ("└─Physical start: 0x", info.memory.phys_start),
        ("└─Virtual start: 0x", info.memory.virt_start),
    ];

    draw_categories(out, width, theme, categories)?;

    Ok(())
}

fn draw_categories(
    out: &mut Out,
    width: usize,
    theme: Theme,
    categories: &[(&str, u64)],
) -> Result<()> {
    categories.iter().try_for_each(|(s, n)| {
        let align = width - s.chars().count() - digits_count(*n);
        draw!(
            out,
            theme.foreground,
            theme.background,
            "{}{}{:<align$}",
            s,
            n,
            ""
        );

        Ok(())
    })
}

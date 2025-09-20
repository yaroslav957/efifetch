use crate::{Out, display::Theme, info::Info, utils::digit_count};
use core::fmt::Write;
use uefi::Result;

pub fn draw(out: &mut Out, width: usize, theme: Theme, info: &Info) -> Result<()> {
    out.set_cursor_position(0, 3)?;
    header(out, width, theme)?;
    label(out, width, theme, info)?;

    Ok(())
}

fn header(out: &mut Out, width: usize, theme: Theme) -> Result<()> {
    let align = width - 14;

    out.set_color(theme.highlight, theme.background)?;
    out.write_str("[Memory Info]:").unwrap();
    out.write_fmt(format_args!("{:<align$}", "")).unwrap();
    out.set_color(theme.foreground, theme.background)
}

fn label(out: &mut Out, width: usize, theme: Theme, info: &Info) -> Result<()> {
    let categories = &[
        ("Total device memory (in mb): ", info.memory.total_memory),
        ("Usable device memory (in mb): ", info.memory.usable_memory),
        ("Phys section start: ", info.memory.phys_start),
        ("Virt section start: ", info.memory.virt_start),
    ];

    out.set_color(theme.foreground, theme.background)?;
    draw_categories(out, width, categories)?;

    Ok(())
}

fn draw_categories(out: &mut Out, width: usize, categories: &[(&str, u64)]) -> Result<()> {
    categories.iter().for_each(|(s, n)| {
        let align = width - s.len() - digit_count(*n);
        out.write_fmt(format_args!("{}{}{:<align$}", s, n, ""))
            .unwrap()
    });

    Ok(())
}

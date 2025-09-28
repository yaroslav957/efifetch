use crate::{
    Out, cursor,
    display::{Category, Page, Theme, main, topbar},
    draw,
};
use core::fmt::Write;
use uefi::Result;

const INDENT: usize = 1;
const LABEL_WIDTH: usize = 14;
const CATEGORIES: &[&str] = &["Cpu", "Memory", "PCI"];
const MARGIN_CPU: usize = LABEL_WIDTH - CATEGORIES[0].len() - INDENT;
const MARGIN_MEM: usize = LABEL_WIDTH - CATEGORIES[1].len() - INDENT;
const MARGIN_PCI: usize = LABEL_WIDTH - CATEGORIES[2].len() - INDENT;

pub fn draw(out: &mut Out, width: usize, height: usize, theme: Theme) -> Result<()> {
    let height = height - 4;
    let width = width - 17;

    cursor!(out, 0, 1);
    header(out, width, theme);
    label(out, width, height, theme)?;
    footer(out, width, theme);

    topbar::update(out, theme, Page::Main)?;
    main::update(out, theme, Category::Cpu);

    Ok(())
}

pub fn update(out: &mut Out, theme: Theme, category: Category) {
    clear_categories(out, theme);

    match category {
        Category::Cpu => {
            cursor!(out, 1, 2);
            draw!(
                out,
                theme.topbar_highlite.fg,
                theme.topbar_highlite.bg,
                " Cpu{:<MARGIN_CPU$}",
                ""
            );
        }

        Category::Memory => {
            cursor!(out, 1, 3);
            draw!(
                out,
                theme.topbar_highlite.fg,
                theme.topbar_highlite.bg,
                " Memory{:<MARGIN_MEM$}",
                ""
            );
        }

        Category::PCI => {
            cursor!(out, 1, 4);
            draw!(
                out,
                theme.topbar_highlite.fg,
                theme.topbar_highlite.bg,
                " PCI{:<MARGIN_PCI$}",
                ""
            );
        }
    }
}

fn header(out: &mut Out, width: usize, theme: Theme) {
    draw!(
        out,
        theme.page.fg,
        theme.page.bg,
        "┌{:─<LABEL_WIDTH$}┬{:─<width$}┐",
        "",
        ""
    );
}

fn label(out: &mut Out, width: usize, height: usize, theme: Theme) -> Result<()> {
    (0..height).try_for_each(|i| {
        if let Some(page) = CATEGORIES.get(i) {
            let margin_left = LABEL_WIDTH - INDENT - CATEGORIES[i].len();
            draw!(
                out,
                theme.page.fg,
                theme.page.bg,
                "│{:<INDENT$}{}{:<margin_left$}│{:<width$}│",
                "",
                page,
                "",
                ""
            );
        } else {
            draw!(
                out,
                theme.page.fg,
                theme.page.bg,
                "│{:<LABEL_WIDTH$}│{:<width$}│",
                "",
                ""
            );
        }

        Ok(())
    })
}

fn footer(out: &mut Out, width: usize, theme: Theme) {
    draw!(
        out,
        theme.page.fg,
        theme.page.bg,
        "└{:─<LABEL_WIDTH$}┴{:─<width$}┘",
        "",
        ""
    );
}

fn clear_categories(out: &mut Out, theme: Theme) {
    cursor!(out, 1, 2);
    draw!(out, theme.page.fg, theme.page.bg, " Cpu{:<MARGIN_CPU$}", "");
    cursor!(out, 1, 3);
    draw!(
        out,
        theme.page.fg,
        theme.page.bg,
        " Memory{:<MARGIN_MEM$}",
        ""
    );
    cursor!(out, 1, 4);
    draw!(out, theme.page.fg, theme.page.bg, " PCI{:<MARGIN_PCI$}", "");
}

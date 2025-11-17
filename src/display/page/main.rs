use crate::{
    Out, cursor,
    display::{Category, Display},
    draw,
};
use core::fmt::{Arguments, Write};
use uefi::Result;

const INDENT: usize = 1;
const LABEL_WIDTH: usize = 14;
const CATEGORIES: [&str; 3] = ["Cpu", "Memory", "PCI"];
const CATEGORIES_MARGIN: [usize; 3] = [
    margin(CATEGORIES[0]),
    margin(CATEGORIES[1]),
    margin(CATEGORIES[2]),
];

impl Display {
    pub fn draw_main(&self, out: &mut Out) -> Result<()> {
        cursor!(out, 0, 1);
        self.header_main(out);
        self.label_main(out);
        self.footer_main(out);
        self.update_main(out);

        Ok(())
    }

    pub fn update_main(&self, out: &mut Out) {
        self.clear_categories(out);

        match self.category {
            Category::Cpu => self.update_main_category(
                out,
                2,
                format_args!(
                    "{:<INDENT$}{}{:<margin$}",
                    "",
                    CATEGORIES[0],
                    "",
                    margin = CATEGORIES_MARGIN[0]
                ),
            ),
            Category::Memory => self.update_main_category(
                out,
                3,
                format_args!(
                    "{:<INDENT$}{}{:<margin$}",
                    "",
                    CATEGORIES[1],
                    "",
                    margin = CATEGORIES_MARGIN[1],
                ),
            ),

            Category::PCI => self.update_main_category(
                out,
                4,
                format_args!(
                    "{:<INDENT$}{}{:<margin$}",
                    "",
                    CATEGORIES[2],
                    "",
                    margin = CATEGORIES_MARGIN[2]
                ),
            ),
        }
    }

    fn update_main_category(&self, out: &mut Out, pos: usize, label: Arguments) {
        cursor!(out, INDENT, pos);
        draw!(
            out,
            self.theme.topbar_highlite.fg,
            self.theme.topbar_highlite.bg,
            "{label}",
        );
    }

    fn header_main(&self, out: &mut Out) {
        let width = self.resolution.width - LABEL_WIDTH - INDENT * 3;

        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            "┌{:─<LABEL_WIDTH$}┬{:─<width$}┐",
            "",
            ""
        );
    }

    fn label_main(&self, out: &mut Out) {
        let width = self.resolution.width - LABEL_WIDTH - INDENT * 3;
        let height = self.resolution.height - INDENT * 4;

        (0..height).for_each(|i| {
            if let Some(category) = CATEGORIES.get(i)
                && let Some(margin) = CATEGORIES_MARGIN.get(i)
            {
                draw!(
                    out,
                    self.theme.page.fg,
                    self.theme.page.bg,
                    "│{:<INDENT$}{category}{:<margin$}│{:<width$}│",
                    "",
                    "",
                    ""
                );
            } else {
                draw!(
                    out,
                    self.theme.page.fg,
                    self.theme.page.bg,
                    "│{:<LABEL_WIDTH$}│{:<width$}│",
                    "",
                    ""
                );
            }
        });
    }

    fn footer_main(&self, out: &mut Out) {
        let width = self.resolution.width - LABEL_WIDTH - INDENT * 3;

        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            "└{:─<LABEL_WIDTH$}┴{:─<width$}┘",
            "",
            ""
        );
    }

    fn clear_categories(&self, out: &mut Out) {
        (2..=4).for_each(|i| {
            if let Some(category) = CATEGORIES.get(i - 2)
                && let Some(margin) = CATEGORIES_MARGIN.get(i - 2)
            {
                cursor!(out, INDENT, i);
                draw!(
                    out,
                    self.theme.page.fg,
                    self.theme.page.bg,
                    "{:<INDENT$}{category}{:<margin$}",
                    "",
                    ""
                );
            }
        });
    }
}

const fn margin(label: &str) -> usize {
    // Waiting for const `.chars().count()` day 1
    LABEL_WIDTH - label.len() - INDENT
}

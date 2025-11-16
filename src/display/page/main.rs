use crate::{
    Out, cursor,
    display::{Category, Display},
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

impl Display {
    pub fn draw_main(&self, out: &mut Out) -> Result<()> {
        cursor!(out, 0, 1);
        self.header_main(out);
        self.label_main(out)?;
        self.footer_main(out);
        self.update_main(out, Category::Cpu);

        Ok(())
    }

    pub fn update_main(&self, out: &mut Out, category: Category) {
        self.clear_categories(out);

        match category {
            Category::Cpu => {
                cursor!(out, 1, 2);
                draw!(
                    out,
                    self.theme.topbar_highlite.fg,
                    self.theme.topbar_highlite.bg,
                    " Cpu{:<MARGIN_CPU$}",
                    ""
                );
            }

            Category::Memory => {
                cursor!(out, 1, 3);
                draw!(
                    out,
                    self.theme.topbar_highlite.fg,
                    self.theme.topbar_highlite.bg,
                    " Memory{:<MARGIN_MEM$}",
                    ""
                );
            }

            Category::PCI => {
                cursor!(out, 1, 4);
                draw!(
                    out,
                    self.theme.topbar_highlite.fg,
                    self.theme.topbar_highlite.bg,
                    " PCI{:<MARGIN_PCI$}",
                    ""
                );
            }
        }
    }

    fn header_main(&self, out: &mut Out) {
        let width = self.resolution.width - 17;

        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            "┌{:─<LABEL_WIDTH$}┬{:─<width$}┐",
            "",
            ""
        );
    }

    fn label_main(&self, out: &mut Out) -> Result<()> {
        let width = self.resolution.width - 17;
        let height = self.resolution.height - 4;

        (0..height).try_for_each(|i| {
            if let Some(page) = CATEGORIES.get(i) {
                let margin_left = LABEL_WIDTH - INDENT - CATEGORIES[i].len();
                draw!(
                    out,
                    self.theme.page.fg,
                    self.theme.page.bg,
                    "│{:<INDENT$}{}{:<margin_left$}│{:<width$}│",
                    "",
                    page,
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

            Ok(())
        })
    }

    fn footer_main(&self, out: &mut Out) {
        let width = self.resolution.width - 17;

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
        cursor!(out, 1, 2);
        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            " Cpu{:<MARGIN_CPU$}",
            ""
        );
        cursor!(out, 1, 3);
        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            " Memory{:<MARGIN_MEM$}",
            ""
        );
        cursor!(out, 1, 4);
        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            " PCI{:<MARGIN_PCI$}",
            ""
        );
    }
}

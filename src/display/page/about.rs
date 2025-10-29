use crate::{
    Out, cursor,
    display::{Display, Page},
    draw,
};
use core::fmt::Write;
use uefi::Result;

impl Display {
    pub fn draw_about(&self, out: &mut Out) -> Result<()> {
        cursor!(out, 0, 1);
        self.header_about(out);
        self.label_about(out)?;
        self.footer_about(out);

        self.update_topbar(out, Page::About)?;

        Ok(())
    }

    fn header_about(&self, out: &mut Out) {
        let width = self.resolution.width - 2;

        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            "┌{:─<width$}┐",
            ""
        );
    }

    fn label_about(&self, out: &mut Out) -> Result<()> {
        let width = self.resolution.width - 2;
        let height = self.resolution.height - 4;

        (0..height).for_each(|_| {
            draw!(
                out,
                self.theme.page.fg,
                self.theme.page.bg,
                "│{:<width$}│",
                ""
            );
        });

        Ok(())
    }

    fn footer_about(&self, out: &mut Out) {
        let width = self.resolution.width - 2;

        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            "└{:─<width$}┘",
            ""
        );
    }
}

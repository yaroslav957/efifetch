use crate::{
    Out, cursor,
    display::{Display, page::Page},
    draw,
};
use core::fmt::Write;
use uefi::Result;

const INDENT: usize = 1;
const PAGES: &[&str] = &["Main", "About", "Exit"];

impl Display {
    pub fn draw_topbar(&self, out: &mut Out) -> Result<()> {
        let margin = self.resolution.width - PAGES.iter().map(|p| p.len() + 2).sum::<usize>();

        self.draw_pages(out, PAGES)?;
        draw!(
            out,
            self.theme.topbar.fg,
            self.theme.topbar.bg,
            "{:<margin$}",
            ""
        );

        Ok(())
    }

    pub fn update_topbar(&self, out: &mut Out, page: Page) -> Result<()> {
        cursor!(out, 0, 0);
        self.draw_pages(out, PAGES)?;

        match page {
            Page::Main => {
                cursor!(out, 0, 0);
                draw!(
                    out,
                    self.theme.topbar_highlite.fg,
                    self.theme.topbar_highlite.bg,
                    "{:<INDENT$}Main{:<INDENT$}",
                    "",
                    ""
                );

                Ok(())
            }

            Page::About => {
                cursor!(out, 6, 0);
                draw!(
                    out,
                    self.theme.topbar_highlite.fg,
                    self.theme.topbar_highlite.bg,
                    "{:<INDENT$}About{:<INDENT$}",
                    "",
                    ""
                );

                Ok(())
            }

            Page::Exit => {
                cursor!(out, 13, 0);
                draw!(
                    out,
                    self.theme.topbar_highlite.fg,
                    self.theme.topbar_highlite.bg,
                    "{:<INDENT$}Exit{:<INDENT$}",
                    "",
                    ""
                );

                Ok(())
            }
        }
    }

    fn draw_pages(&self, out: &mut Out, pages: &[&str]) -> Result<()> {
        pages.iter().try_for_each(|p| {
            draw!(
                out,
                self.theme.page_highlite.fg,
                self.theme.topbar.bg,
                "{:<INDENT$}{}",
                "",
                &p[0..1]
            );
            draw!(
                out,
                self.theme.topbar.fg,
                self.theme.topbar.bg,
                "{}{:<INDENT$}",
                &p[1..],
                ""
            );

            Ok(())
        })
    }
}

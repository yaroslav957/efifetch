use crate::{
    Out, cursor,
    display::{Display, page::Page},
    draw,
};
use core::fmt::Write;

const INDENT: usize = 1;
const PAGES: &[&str] = &["Main", "About", "Exit"];

impl Display {
    pub fn draw_topbar(&self, out: &mut Out) {
        let margin = self.resolution.width
            - PAGES
                .iter()
                .map(|p| p.chars().count() + INDENT * 2)
                .sum::<usize>();

        self.draw_pages(out, PAGES);
        draw!(
            out,
            self.theme.topbar.fg,
            self.theme.topbar.bg,
            "{:<margin$}",
            ""
        );
    }

    pub fn update_topbar(&self, out: &mut Out) {
        cursor!(out, 0, 0);
        self.draw_pages(out, PAGES);

        match self.page {
            Page::Main => self.update_topbar_page(out, 0, "Main"),
            Page::About => self.update_topbar_page(out, 6, "About"),
            Page::Exit => self.update_topbar_page(out, 13, "Exit"),
        }
    }

    fn update_topbar_page(&self, out: &mut Out, pos: usize, label: &str) {
        cursor!(out, pos, 0);
        draw!(
            out,
            self.theme.topbar_highlite.fg,
            self.theme.topbar_highlite.bg,
            "{:<INDENT$}{label}{:<INDENT$}",
            "",
            ""
        );
    }

    fn draw_pages(&self, out: &mut Out, pages: &[&str]) {
        pages.iter().for_each(|p| {
            draw!(
                out,
                self.theme.topbar_keys_highlite.fg,
                self.theme.topbar_keys_highlite.bg,
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
        });
    }
}

use crate::{
    Out,
    consts::{AUTHOR, DESCRIPTION, LICENSE, LOGO, MSRV, REPOSITORY, VERSION},
    cursor,
    display::Display,
    draw,
};
use core::fmt::Write;

const INDENT: usize = 1;
const LABELS: &[&str] = &[
    "MSRV:",
    "License:",
    "Version:",
    "Made by:",
    "Repo:",
    "Description:",
];

impl Display {
    pub fn draw_about(&self, out: &mut Out) {
        cursor!(out, 0, 1);
        self.header_about(out);
        self.logo_about(out);
        self.label_about(out);
        self.footer_about(out);
    }

    fn header_about(&self, out: &mut Out) {
        let width = self.resolution.width - INDENT * 2;

        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            "┌{:─<width$}┐",
            ""
        );
    }

    fn logo_about(&self, out: &mut Out) {
        LOGO.lines().for_each(|line| {
            let width = (self.resolution.width - INDENT * 2 - line.chars().count()) / 2;
            draw!(
                out,
                self.theme.page.fg,
                self.theme.page.bg,
                "│{:<width$}{line}{:<width$}│",
                "",
                ""
            );
        });
    }

    fn label_about(&self, out: &mut Out) {
        let width = self.resolution.width - INDENT * 2;
        let height = self.resolution.height - INDENT * 4 - 15;

        let content = [
            MSRV,
            LICENSE,
            VERSION,
            AUTHOR,
            &REPOSITORY[8..],
            DESCRIPTION,
        ];

        (0..height).for_each(|i| {
            if let Some(label_name) = LABELS.get(i)
                && let Some(label_content) = content.get(i)
            {
                let label_len = label_content.chars().count() + label_name.chars().count();
                let margin_right = width - (label_len + INDENT * 2);

                draw!(
                    out,
                    self.theme.page.fg,
                    self.theme.page.bg,
                    "│{:<INDENT$}{}",
                    "",
                    label_name
                );
                draw!(
                    out,
                    self.theme.page_highlite.fg,
                    self.theme.page_highlite.bg,
                    "{:<INDENT$}{}",
                    "",
                    label_content,
                );
                draw!(
                    out,
                    self.theme.page.fg,
                    self.theme.page.bg,
                    "{:<margin_right$}│",
                    ""
                );
            } else {
                draw!(
                    out,
                    self.theme.page.fg,
                    self.theme.page.bg,
                    "│{:<width$}│",
                    ""
                );
            }
        })
    }

    fn footer_about(&self, out: &mut Out) {
        let width = self.resolution.width - INDENT * 2;

        draw!(
            out,
            self.theme.page.fg,
            self.theme.page.bg,
            "└{:─<width$}┘",
            ""
        );
    }
}

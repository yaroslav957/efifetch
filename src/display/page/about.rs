use crate::{
    Out,
    consts::{AUTHOR, DESCRIPTION, LICENSE, MSRV, REPOSITORY, VERSION},
    cursor,
    display::Display,
    draw,
    info::Info,
};
use core::fmt::Write;
use uefi::Result;

const INDENT: usize = 1;
const BLANK_LINE: &str = "";
const LABELS: &[&str] = &[
    "Firmware revision:",
    "Firmware vendor:",
    "UEFI revesion:",
    BLANK_LINE,
    "MSRV:",
    "License:",
    "Version:",
    "Made by:",
    "Repo:",
    "Description:",
    BLANK_LINE,
    "Theme:",
];

impl Display {
    pub fn draw_about(&self, out: &mut Out, info: &Info) -> Result<()> {
        cursor!(out, 0, 1);
        self.header_about(out);
        self.label_about(out, info)?;
        self.footer_about(out);

        Ok(())
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

    fn label_about(&self, out: &mut Out, info: &Info) -> Result<()> {
        let width = self.resolution.width - INDENT * 2;
        let height = self.resolution.height - INDENT * 4;
        let revision = info.firmware.revision.as_str();
        let vendor = info.firmware.vendor;
        let uefi_revision = info.firmware.uefi_revision.as_str();
        let theme = self.theme.name;

        let content = [
            revision,
            vendor,
            uefi_revision,
            BLANK_LINE,
            MSRV,
            LICENSE,
            VERSION,
            AUTHOR,
            &REPOSITORY[8..],
            DESCRIPTION,
            BLANK_LINE,
            theme,
        ];

        (0..height).try_for_each(|i| {
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

            Ok(())
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

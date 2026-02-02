use crate::{
    consts::env::{
        AUTHOR, DESCRIPTION, LICENSE, LOGO, MSRV, NAME, REPOSITORY, VERSION,
    },
    error::Result,
    info::Info,
    tui::{Page, Theme},
};
use core::{fmt::Write, str::Lines};
use uefi::{
    Error, ResultExt, Status,
    boot::{ScopedProtocol, wait_for_event},
    proto::console::text::{Color, Input, Output, OutputMode},
};

const TL: char = '┌';
const TR: char = '┐';
const VL: char = '│';
const BL: char = '└';
const BR: char = '┘';
const PAGE_NAMES: [&str; 7] = [
    " Main ",
    " Firmware ",
    " Cpu ",
    " Ram ",
    " Pci ",
    " Acpi ",
    " Exit ",
];
const MAIN_PAGE: [(&str, &str); 7] = [
    ("Name: ", NAME),
    ("Made by: ", AUTHOR),
    ("Description: ", DESCRIPTION),
    ("Build version: ", VERSION),
    ("License: ", LICENSE),
    ("Repo: ", REPOSITORY),
    ("Minimal Rust version: ", MSRV),
];
const EXIT_PAGE: [(&str, &str); 1] = [("", "")];

pub struct Canvas {
    pub inp: ScopedProtocol<Input>,
    pub out: ScopedProtocol<Output>,
    pub mode: OutputMode,
    pub theme: Theme,
    pub page: Page,
}

impl Canvas {
    pub fn new(
        inp: ScopedProtocol<Input>,
        mut out: ScopedProtocol<Output>,
        theme: Theme,
    ) -> Result<Self> {
        let mode = out.modes().min().ok_or(Error::new(
            Status::UNSUPPORTED,
            "The are no available resolutions",
        ))?;
        let page = Page::default();

        out.set_mode(mode)?;
        out.clear()?;
        out.enable_cursor(false).ok();

        Ok(Self {
            inp,
            out,
            mode,
            theme,
            page,
        })
    }

    pub fn draw_topbar(&mut self) -> Result<&mut Self> {
        let topbar_margin = self.mode.columns()
            - PAGE_NAMES
                .iter()
                .map(|name| name.chars().count())
                .sum::<usize>();

        self.out.set_color(
            self.theme.topbar.foreground,
            self.theme.topbar.background,
        )?;

        PAGE_NAMES
            .iter()
            .try_for_each(|page| write!(self.out, "{page}"))?;

        write!(self.out, "{:<topbar_margin$}", "")?;

        Ok(self)
    }

    pub fn draw_grid(&mut self) -> Result<&mut Self> {
        let topgrid_margin = self.grid_margin(TL, TR);
        let midgrid_margin = self.grid_margin(VL, VL);
        let botgrid_margin = self.grid_margin(BL, BR);

        self.out.set_color(
            self.theme.grid.foreground,
            self.theme.grid.background,
        )?;

        write!(self.out, "{TL}{:─<topgrid_margin$}{TR}", "")?;

        // Skip 4 rows: topbar, top border, bottom border,
        // and reserved row for newline
        // to prevent topbar from being pushed off-screen
        (0..self.mode.rows() - 4).try_for_each(|_| {
            write!(self.out, "{VL}{:<midgrid_margin$}{VL}", "")
        })?;

        write!(self.out, "{BL}{:─<botgrid_margin$}{BR}", "")?;

        Ok(self)
    }

    pub fn update_grid(&mut self, info: Info) -> Result<()> {
        let logo = LOGO.lines();
        let firmware_page = [
            ("Revision: ", info.firmware.revision().as_str()),
            ("Vendor: ", info.firmware.vendor()),
            ("uefi revision: ", info.firmware.uefi_revision().as_str()),
        ];
        let cpu_page = [("")];
        let ram_page = [
            ("All memory: ", info.memory.total_memory().as_str()),
            ("Usable memory: ", info.memory.usable_memory().as_str()),
            ("Physical ptr start: ", info.memory.phys_start().as_str()),
            ("Virtual ptr start: ", info.memory.virt_start().as_str()),
        ];
        let pci_page = [("")];
        let acpi_page = [("")];

        loop {
            let mut events =
                [self.inp.wait_for_key_event().ok_or(Error::new(
                    Status::UNSUPPORTED,
                    "Input device not available or unsupported",
                ))?];
            wait_for_event(&mut events).discard_errdata()?;

            // to be done
        }

        Ok(())
    }

    fn draw_page(&mut self) -> Result<()> {
        self.out.set_cursor_position(0, 0)?;

        Ok(())
    }

    fn draw_logo(&mut self, logo: Lines) -> Result<()> {
        Ok(())
    }

    fn grid_margin(&mut self, left: char, right: char) -> usize {
        self.mode.columns() - (left.len_utf16() + right.len_utf16())
    }
}

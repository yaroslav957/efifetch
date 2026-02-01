use crate::{error::Result, tui::Theme};
use core::fmt::Write;
use uefi::{
    Error, Status,
    boot::ScopedProtocol,
    proto::console::text::{Color, Input, Output, OutputMode},
};

const PAGE_NAMES: [&str; 6] =
    [" Main ", " FIRMWARE ", " CPU ", " RAM ", " PCI ", " Exit "];
const TL: char = '┌';
const TR: char = '┐';
const VL: char = '│';
const BL: char = '└';
const BR: char = '┘';

pub struct Canvas {
    inp: ScopedProtocol<Input>,
    out: ScopedProtocol<Output>,
    mode: OutputMode,
    theme: Theme,
    /*    cursor: Page, */
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

        out.set_mode(mode)?;
        out.clear()?;
        out.enable_cursor(false).ok();

        Ok(Self {
            inp,
            out,
            mode,
            theme,
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
        (0..self.mode.rows() - 4).try_for_each(|_| {
            write!(self.out, "{VL}{:<midgrid_margin$}{VL}", "")
        })?;
        write!(self.out, "{BL}{:─<botgrid_margin$}{BR}", "")?;

        Ok(self)
    }

    fn grid_margin(&mut self, left: char, right: char) -> usize {
        self.mode.columns() - (left.len_utf16() + right.len_utf16())
    }
}

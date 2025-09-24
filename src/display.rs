mod page;
mod theme;
mod topbar;

use crate::{
    Out,
    display::{
        page::{main /* memory*/},
        theme::Theme,
    },
    info::Info,
    utils::resolution,
};
use uefi::Result;

#[macro_export]
macro_rules! draw {
    ($out:expr, $fg:expr, $bg:expr, $($arg:tt)*) => {
        $out.set_color($fg, $bg).unwrap();
        $out.write_fmt(format_args!($($arg)*)).unwrap();
    };
}

#[macro_export]
macro_rules! cursor {
    ($out:expr, $column:expr, $row:expr) => {
        $out.set_cursor_position($column, $row).unwrap()
    };
}

pub struct Display {
    width: usize,
    height: usize,
    theme: Theme,
}

#[allow(dead_code)]
impl Display {
    pub fn new(out: &mut Out) -> Result<Self> {
        let [width, height] = resolution(out)?;
        let theme = Theme::default();

        Ok(Self {
            width,
            height,
            theme,
        })
    }

    pub fn change_theme(&mut self, theme: Theme) {
        self.theme = theme
    }

    pub fn topbar(&self, out: &mut Out) -> Result<()> {
        topbar::draw(out, self.width, self.theme)
    }

    pub fn main_page(&self, out: &mut Out) -> Result<()> {
        main::draw(out, self.width, self.height, self.theme)
    }

    pub fn memory_page(&self, out: &mut Out, info: &Info) -> Result<()> {
        Ok(()) // memory::draw(out, self.width, self.theme, info)
    }
}

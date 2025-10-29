pub mod page;
pub mod theme;
pub mod topbar;

use crate::{
    Out,
    display::{
        page::{Category, Page},
        theme::Theme,
    },
};
use uefi::{Error, Result, Status};

const MIN_CONSOLE_WIDTH: usize = 80;
const MIN_CONSOLE_HEIGHT: usize = 25;

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
    theme: Theme,
    page: Page,
    category: Category,
    resolution: Resolution,
}

#[allow(dead_code)]
impl Display {
    fn clear(out: &mut Out) -> Result<()> {
        out.clear()
    }

    pub fn new(out: &mut Out) -> Result<Self> {
        Display::clear(out)?;

        let theme = Theme::default();
        let page = Page::default();
        let category = Category::default();
        let resolution = Resolution::new(out)?;

        Ok(Self {
            theme,
            page,
            category,
            resolution,
        })
    }

    pub fn page(&self) -> Page {
        self.page
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn change_theme(&mut self, theme: Theme) {
        self.theme = theme
    }

    pub fn next_category(&mut self, out: &mut Out) {
        match self.category() {
            Category::Cpu => self.category = Category::Memory,
            Category::Memory => self.category = Category::PCI,
            Category::PCI => self.category = Category::Cpu,
        }

        self.update_main(out, self.category);
    }

    pub fn prev_category(&mut self, out: &mut Out) {
        match self.category() {
            Category::Cpu => self.category = Category::PCI,
            Category::Memory => self.category = Category::Cpu,
            Category::PCI => self.category = Category::Memory,
        }

        self.update_main(out, self.category);
    }

    pub fn main_page(&mut self, out: &mut Out) -> Result<()> {
        self.page = Page::Main;
        self.draw_main(out)
    }

    pub fn about_page(&mut self, out: &mut Out) -> Result<()> {
        self.page = Page::About;
        self.draw_about(out)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

impl Resolution {
    fn minimize(out: &mut Out) -> Result<()> {
        let min_mode = out
            .modes()
            .min()
            .ok_or(Error::new(Status::UNSUPPORTED, ()))?;

        out.set_mode(min_mode)
    }

    pub fn new(out: &mut Out) -> Result<Self> {
        Resolution::minimize(out)?;

        let mode = out
            .current_mode()?
            .ok_or(Error::new(Status::UNSUPPORTED, ()))?;
        let width = mode.columns();
        let height = mode.rows();

        if width < MIN_CONSOLE_WIDTH || height < MIN_CONSOLE_HEIGHT {
            Err(Error::new(Status::UNSUPPORTED, ()))
        } else {
            Ok(Self { width, height })
        }
    }
}

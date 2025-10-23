pub mod page;
pub mod theme;
pub mod topbar;

use crate::{
    Out,
    display::{
        page::{Category, Page, about, main},
        theme::Theme,
    },
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
    page: Page,
    category: Category,
}

#[allow(dead_code)]
impl Display {
    pub fn new(out: &mut Out) -> Result<Self> {
        let [width, height] = resolution(out)?;
        let theme = Theme::default();
        let page = Page::default();
        let category = Category::default();

        Ok(Self {
            width,
            height,
            theme,
            page,
            category,
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
        match self.category {
            Category::Cpu => self.category = Category::Memory,
            Category::Memory => self.category = Category::PCI,
            Category::PCI => self.category = Category::Cpu,
        }

        main::update(out, self.theme, self.category);
    }

    pub fn prev_category(&mut self, out: &mut Out) {
        match self.category {
            Category::Cpu => self.category = Category::PCI,
            Category::Memory => self.category = Category::Cpu,
            Category::PCI => self.category = Category::Memory,
        }

        main::update(out, self.theme, self.category);
    }

    pub fn topbar(&self, out: &mut Out) -> Result<()> {
        topbar::draw(out, self.width, self.theme)
    }

    pub fn default_page(&mut self, out: &mut Out) -> Result<()> {
        self.page = Page::default();

        about::draw(out, self.width, self.height, self.theme)
    }

    pub fn main_page(&mut self, out: &mut Out) -> Result<()> {
        self.page = Page::Main;

        main::draw(out, self.width, self.height, self.theme)
    }

    pub fn about_page(&mut self, out: &mut Out) -> Result<()> {
        self.page = Page::About;

        about::draw(out, self.width, self.height, self.theme)
    }
}

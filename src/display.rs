use crate::{Out, utils::resolution};
use uefi::{Result, proto::console::text::Color};

const EFIFETCH_THEME: Theme = Theme::new(Color::Red, Color::Black, Color::LightRed);
pub const COMMON_THEME: Theme = Theme::new(Color::Black, Color::LightGray, Color::Red);
pub const FALLOUT_THEME: Theme = Theme::new(Color::Green, Color::Black, Color::LightGreen);

mod start_screen;
mod top_bar;

pub struct Display {
    width: usize,
    height: usize,
    theme: Theme,
}

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

    pub fn top_bar(&self, out: &mut Out) -> Result<()> {
        top_bar::draw(out, self.width, self.theme)
    }

    pub fn start_screen(&self, out: &mut Out) -> Result<()> {
        start_screen::draw(out, self.width, self.height, self.theme)
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    foreground: Color,
    background: Color,
    highlight: Color,
}

impl Theme {
    pub const fn new(foreground: Color, background: Color, highlight: Color) -> Self {
        Self {
            foreground,
            background,
            highlight,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        EFIFETCH_THEME
    }
}

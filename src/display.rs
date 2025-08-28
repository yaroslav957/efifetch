use crate::{Out, utils::resolution};
use uefi::{Result, proto::console::text::Color};

const PANELS_FOREGROUND: Color = Color::White;
const PANELS_BACKGROUND: Color = Color::Black;

const BAR_FOREGROUND: Color = Color::White;
const BAR_BACKGROUND: Color = Color::Blue;
const BAR_HIGHLIGHT: Color = Color::LightRed;

const SCREEN_FOREGROUND: Color = Color::Black;
const SCREEN_BACKGROUND: Color = Color::LightGray;

mod bottom_bar;
mod start_screen;
mod top_bar;

pub struct Display {
    width: usize,
    height: usize,
}

impl Display {
    pub fn new(out: &mut Out) -> Result<Self> {
        let [width, height] = resolution(out)?;
        Ok(Self { width, height })
    }

    pub fn topbar(&self, out: &mut Out) -> Result<()> {
        top_bar::draw(out, self.width)
    }

    pub fn bottombar(&self, out: &mut Out) -> Result<()> {
        bottom_bar::draw(out, self.width)
    }

    pub fn startscreen(&self, out: &mut Out) -> Result<()> {
        start_screen::draw(out, self.width, self.height)
    }
}

use crate::{Out, utils::resolution};
use uefi::{Result, proto::console::text::Color};

const PANELS_FOREGROUND: Color = Color::Black;
const PANELS_HIGHLIGHT: Color = Color::Red;
const PANELS_BACKGROUND: Color = Color::LightGray;

mod bottombar;
mod topbar;

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
        topbar::draw(out, self.width)
    }

    pub fn bottombar(&self, out: &mut Out) -> Result<()> {
        bottombar::draw(out, self.width)
    }
}

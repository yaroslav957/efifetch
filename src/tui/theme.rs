use uefi::proto::console::text::Color;

pub const DEFAULT: Theme = Theme::new(
    "Efifetch (default)",
    Palette::new(Color::Red, Color::Black),
    Palette::new(Color::Black, Color::Red),
    Palette::new(Color::Red, Color::Black),
    Palette::new(Color::LightRed, Color::Black),
);

pub const FALLOUT: Theme = Theme::new(
    "Fallout",
    Palette::new(Color::Green, Color::Black),
    Palette::new(Color::Black, Color::Green),
    Palette::new(Color::Green, Color::Black),
    Palette::new(Color::LightGreen, Color::Black),
);

pub const CLASSIC: Theme = Theme::new(
    "Classic",
    Palette::new(Color::LightGray, Color::Blue),
    Palette::new(Color::Blue, Color::LightGray),
    Palette::new(Color::Blue, Color::LightGray),
    Palette::new(Color::Blue, Color::LightGray),
);

#[derive(Clone, Copy)]
pub struct Palette {
    pub foreground: Color,
    pub background: Color,
}

impl Palette {
    pub const fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub topbar: Palette,
    pub topbar_hl: Palette,
    pub grid: Palette,
    pub grid_hl: Palette,
}

impl Theme {
    pub const fn new(
        name: &'static str,
        topbar: Palette,
        topbar_hl: Palette,
        grid: Palette,
        grid_hl: Palette,
    ) -> Self {
        Self {
            name,
            topbar,
            topbar_hl,
            grid,
            grid_hl,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        CLASSIC
    }
}

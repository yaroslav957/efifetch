use uefi::proto::console::text::Color;

pub const EFIFETCH_THEME: Theme = Theme::new(
    Palette::new(Color::Red, Color::Black),
    Palette::new(Color::Red, Color::Black),
    Palette::new(Color::LightRed, Color::Black),
    Palette::new(Color::Black, Color::Red),
);

pub const COMMON_THEME: Theme = Theme::new(
    Palette::new(Color::Blue, Color::LightGray),
    Palette::new(Color::LightGray, Color::Blue),
    Palette::new(Color::White, Color::LightGray),
    Palette::new(Color::White, Color::LightGray),
);

#[derive(Clone, Copy)]
pub struct Theme {
    pub page: Palette,
    pub topbar: Palette,
    pub page_highlite: Palette,
    pub topbar_highlite: Palette,
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub fg: Color,
    pub bg: Color,
}

impl Theme {
    pub const fn new(
        page: Palette,
        topbar: Palette,
        page_highlite: Palette,
        topbar_highlite: Palette,
    ) -> Self {
        Self {
            page,
            topbar,
            page_highlite,
            topbar_highlite,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        EFIFETCH_THEME
    }
}

impl Palette {
    pub const fn new(fg: Color, bg: Color) -> Self {
        Self { fg, bg }
    }
}

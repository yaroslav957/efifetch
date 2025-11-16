use uefi::proto::console::text::Color;

pub const EFIFETCH_THEME: Theme = Theme::new(
    "EFIFETCH_THEME",
    Palette::new(Color::Red, Color::Black),
    Palette::new(Color::Red, Color::Black),
    Palette::new(Color::LightRed, Color::Black),
    Palette::new(Color::Black, Color::Red),
    Palette::new(Color::LightRed, Color::Black),
);

pub const COMMON_THEME: Theme = Theme::new(
    "COMMON_THEME",
    Palette::new(Color::Blue, Color::LightGray),
    Palette::new(Color::LightGray, Color::Blue),
    Palette::new(Color::Black, Color::LightGray),
    Palette::new(Color::White, Color::LightGray),
    Palette::new(Color::White, Color::Blue),
);

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub page: Palette,
    pub topbar: Palette,
    pub page_highlite: Palette,
    pub topbar_highlite: Palette,
    pub topbar_keys_highlite: Palette,
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub fg: Color,
    pub bg: Color,
}

impl Theme {
    pub const fn new(
        name: &'static str,
        page: Palette,
        topbar: Palette,
        page_highlite: Palette,
        topbar_highlite: Palette,
        topbar_keys_highlite: Palette,
    ) -> Self {
        Self {
            name,
            page,
            topbar,
            page_highlite,
            topbar_highlite,
            topbar_keys_highlite,
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

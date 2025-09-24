use uefi::proto::console::text::Color;

pub const EFIFETCH_THEME: Theme = Theme::new(
    Color::Red,
    Color::Black,
    Color::Red,
    Color::Black,
    Color::LightRed,
    Color::Black,
);

pub const FALLOUT_THEME: Theme = Theme::new(
    Color::Green,
    Color::Black,
    Color::Green,
    Color::Black,
    Color::LightGreen,
    Color::Black,
);

pub const COMMON_THEME: Theme = Theme::new(
    Color::LightGray,
    Color::Blue,
    Color::Blue,
    Color::LightGray,
    Color::White,
    Color::LightGray,
);

#[derive(Clone, Copy)]
pub struct Theme {
    pub topbar_fg: Color,
    pub topbar_bg: Color,
    pub page_fg: Color,
    pub page_bg: Color,
    pub highlight_fg: Color,
    pub highlight_bg: Color,
}

impl Theme {
    pub const fn new(
        topbar_fg: Color,
        topbar_bg: Color,
        page_fg: Color,
        page_bg: Color,
        highlight_fg: Color,
        highlight_bg: Color,
    ) -> Self {
        Self {
            topbar_fg,
            topbar_bg,
            page_fg,
            page_bg,
            highlight_fg,
            highlight_bg,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        COMMON_THEME
    }
}

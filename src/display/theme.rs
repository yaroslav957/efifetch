use uefi::proto::console::text::Color;

pub const EFIFETCH_THEME: Theme = Theme::new(
    (Color::Red, Color::Black),
    (Color::Red, Color::Black),
    (Color::LightRed, Color::Black),
    (Color::Black, Color::Red),
);

pub const COMMON_THEME: Theme = Theme::new(
    (Color::Blue, Color::LightGray),
    (Color::LightGray, Color::Blue),
    (Color::White, Color::LightGray),
    (Color::White, Color::LightGray),
);

#[derive(Clone, Copy)]
pub struct Theme {
    pub page: (Color, Color),
    pub topbar: (Color, Color),
    pub page_highlite: (Color, Color),
    pub topbar_highlite: (Color, Color),
}

impl Theme {
    pub const fn new(
        page: (Color, Color),
        topbar: (Color, Color),
        page_highlite: (Color, Color),
        topbar_highlite: (Color, Color),
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

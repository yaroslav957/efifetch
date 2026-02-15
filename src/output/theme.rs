use uefi::proto::console::text::Color;

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: &'static str,
    pub logo: Palette,
    pub label: Palette,
    pub content: Palette,
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub foreground: Color,
    pub background: Color,
}

impl Theme {
    pub const RED: Theme = Theme::new(
        "RED",
        Palette::new(Color::LightRed, Color::Black),
        Palette::new(Color::Red, Color::Black),
        Palette::new(Color::LightRed, Color::Black),
    );

    pub const fn new(
        name: &'static str,
        logo: Palette,
        label: Palette,
        content: Palette,
    ) -> Self {
        Self {
            name,
            logo,
            label,
            content,
        }
    }
}

impl Palette {
    pub const fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

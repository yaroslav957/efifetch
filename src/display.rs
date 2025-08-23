use uefi::proto::console::text::Color;

const PANELS_FOREGROUND: Color = Color::Black;
const PANELS_HIGHLIGHT: Color = Color::Red;
const PANELS_BACKGROUND: Color = Color::LightGray;

pub mod topbar;

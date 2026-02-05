pub mod theme;

use crate::{info::Info, output::theme::Theme};
use uefi::{boot::ScopedProtocol, proto::console::text::Output};

#[derive(Clone, Copy, Default)]
pub enum Page {
    #[default]
    Main,
}

pub fn draw(
    stdout: &mut ScopedProtocol<Output>,
    info: Info,
    theme: Theme,
    page: Option<Page>,
) {
    todo!()
}

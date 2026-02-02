use crate::{error::Result, tui::Canvas};

const INDENT: usize = 1;

#[derive(Clone, Copy, Default)]
pub enum Page {
    #[default]
    Main,
    Firmware,
    Cpu,
    Ram,
    Pci,
    Acpi,
    Exit,
}

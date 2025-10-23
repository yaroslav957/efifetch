pub mod about;
pub mod main;
//pub mod memory;

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub enum Page {
    Main,
    #[default]
    About,
    Exit,
}

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub enum Category {
    #[default]
    Cpu,
    Memory,
    PCI,
}

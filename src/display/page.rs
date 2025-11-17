pub mod about;
pub mod main;

#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub enum Page {
    #[default]
    Main,
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

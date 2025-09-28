pub mod about;
pub mod main;
//pub mod memory;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Main,
    About,
    Exit,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Category {
    Cpu,
    Memory,
    PCI,
}

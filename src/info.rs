const LOGO: &str = include_str!("./assets/uefi.logo");
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod cpu;
mod mem;
mod pci;

pub enum Info {
    Cpu(Cpu),
    Pci(Pci),
    Mem(Mem),
}

pub struct Cpu;
pub struct Pci;
pub struct Mem;

impl Info {
    pub fn new() {}
}

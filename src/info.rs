use crate::{
    error::Result,
    info::{firmware::Firmware, memory::Memory},
};

mod firmware;
mod memory;

pub trait InfoItem {
    fn render(&self) -> impl Iterator<Item = (&str, &str)>;
}

#[derive(Clone, Debug)]
pub struct Info {
    pub firmware: Firmware,
    pub memory: Memory,
}

impl Info {
    pub fn new() -> Result<Self> {
        let firmware = Firmware::new()?;
        let memory = Memory::new()?;

        Ok(Self { firmware, memory })
    }
}

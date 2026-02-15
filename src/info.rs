mod date;
mod firmware;
mod memory;

use crate::{
    error::Result,
    info::{date::Date, firmware::Firmware, memory::Memory},
};
//TODO: impl debug for every field
#[derive(Clone)]
pub struct Info {
    pub date: Date,
    pub firmware: Firmware,
    pub memory: Memory,
}

impl Info {
    pub fn new() -> Result<Self> {
        let date = Date::new()?;
        let firmware = Firmware::new()?;
        let memory = Memory::new()?;

        Ok(Self {
            date,
            firmware,
            memory,
        })
    }
}

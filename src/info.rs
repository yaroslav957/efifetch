#![allow(unused)]

mod date;
mod firmware;
mod memory;

use crate::info::{date::Date, firmware::Firmware, memory::Memory};
use uefi::Result;

#[derive(Clone, Copy)]
pub struct Info {
    date: Date,
    firmware: Firmware,
    memory: Memory,
}

impl Info {
    pub fn new() -> Result<Self> {
        let date = Date::new()?;
        let firmware = Firmware::new();
        let memory = Memory::new()?;

        Ok(Self {
            date,
            firmware,
            memory,
        })
    }

    pub fn date(&self) -> Date {
        self.date
    }

    pub fn firmware(&self) -> Firmware {
        self.firmware
    }

    pub fn memory(&self) -> Memory {
        self.memory
    }
}

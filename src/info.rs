mod memory;

use crate::info::memory::Memory;
use uefi::Result;

pub struct Info {
    memory: Memory,
}

impl Info {
    pub fn new() -> Result<Self> {
        let memory = Memory::new()?;

        Ok(Self { memory })
    }
}

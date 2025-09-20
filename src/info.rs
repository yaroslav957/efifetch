mod cpu;
mod memory;

use crate::info::{cpu::Cpu, memory::Memory};
use uefi::Result;

pub struct Info {
    pub cpu: Cpu,
    pub memory: Memory,
}

impl Info {
    pub fn new() -> Result<Self> {
        let memory = Memory::new()?;
        let cpu = Cpu::new();

        Ok(Self { cpu, memory })
    }
}

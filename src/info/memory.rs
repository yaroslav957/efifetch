use crate::utils::U32Buffer;
use uefi::{
    Error, Result, Status,
    boot::{MemoryType, PAGE_SIZE, memory_map},
    mem::memory_map::{MemoryMap, MemoryMapOwned},
};

const MB: u64 = 1024 * 1024;
const MEMORY_TYPES: &[MemoryType] = &[
    MemoryType::CONVENTIONAL,
    MemoryType::RESERVED,
    MemoryType::LOADER_DATA,
    MemoryType::LOADER_CODE,
    MemoryType::BOOT_SERVICES_DATA,
    MemoryType::BOOT_SERVICES_CODE,
    MemoryType::RUNTIME_SERVICES_DATA,
    MemoryType::RUNTIME_SERVICES_CODE,
    MemoryType::PERSISTENT_MEMORY,
    MemoryType::ACPI_RECLAIM,
    MemoryType::ACPI_NON_VOLATILE,
    MemoryType::MMIO,
    MemoryType::MMIO_PORT_SPACE,
];

#[derive(Clone, Copy)]
pub struct Memory {
    total_memory: U32Buffer,
    usable_memory: U32Buffer,
    phys_start: U32Buffer,
    virt_start: U32Buffer,
}

impl Memory {
    pub fn new() -> Result<Self> {
        let map = memory_map(MemoryType::LOADER_DATA)?;
        let total_memory = U32Buffer::new(Memory::count_memory(&map, MEMORY_TYPES));
        let usable_memory = U32Buffer::new(Memory::count_memory(&map, &[MEMORY_TYPES[0]]));
        let (phys_start, virt_start) = {
            let starts = Memory::find_start(&map)?;
            (U32Buffer::new(starts.0), U32Buffer::new(starts.1))
        };

        Ok(Self {
            total_memory,
            usable_memory,
            phys_start,
            virt_start,
        })
    }

    pub fn total_memory(&self) -> U32Buffer {
        self.total_memory
    }

    pub fn usable_memory(&self) -> U32Buffer {
        self.usable_memory
    }

    pub fn phys_start(&self) -> U32Buffer {
        self.phys_start
    }

    pub fn virt_start(&self) -> U32Buffer {
        self.virt_start
    }

    fn count_memory(map: &MemoryMapOwned, types: &[MemoryType]) -> u32 {
        map.entries()
            .filter(|d| types.contains(&d.ty))
            .map(|d| d.page_count * PAGE_SIZE as u64)
            .sum::<u64>()
            .div_ceil(MB) as u32
    }

    fn find_start(map: &MemoryMapOwned) -> Result<(u32, u32)> {
        map.entries()
            .find(|d| d.ty == MemoryType::CONVENTIONAL)
            .map(|d| (d.phys_start as u32, d.virt_start as u32))
            .ok_or(Error::new(Status::UNSUPPORTED, ()))
    }
}

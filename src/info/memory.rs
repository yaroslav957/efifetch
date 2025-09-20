use uefi::{
    Result,
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

#[allow(dead_code)]
pub struct Memory {
    pub total_memory: u64,
    pub usable_memory: u64,
    pub phys_start: u64,
    pub virt_start: u64,
}

impl Memory {
    pub fn new() -> Result<Self> {
        let map = memory_map(MemoryType::LOADER_DATA)?;

        let total_memory = count_memory(&map, MEMORY_TYPES);
        let usable_memory = count_memory(&map, &[MemoryType::CONVENTIONAL]);
        let (phys_start, virt_start) = find_start(&map);

        Ok(Self {
            total_memory,
            usable_memory,
            phys_start,
            virt_start,
        })
    }
}

fn count_memory(map: &MemoryMapOwned, types: &[MemoryType]) -> u64 {
    map.entries()
        .filter(|d| types.contains(&d.ty))
        .map(|d| d.page_count * PAGE_SIZE as u64)
        .sum::<u64>()
        .div_ceil(MB)
}

fn find_start(map: &MemoryMapOwned) -> (u64, u64) {
    map.entries()
        .find(|d| d.ty == MemoryType::CONVENTIONAL)
        .map(|d| (d.phys_start, d.virt_start))
        .unwrap_or_default()
}

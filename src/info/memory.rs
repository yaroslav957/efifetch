use crate::{error::Result, info::InfoItem};

use alloc::{format, string::String};
use uefi::{
    boot::{MemoryType, PAGE_SIZE, memory_map},
    mem::memory_map::{MemoryMap, MemoryMapOwned},
};

const MIB: u64 = 1024 * 1024;

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Memory {
    pub count: String,
}

impl Memory {
    pub fn new() -> Result<Self> {
        let map = memory_map(MemoryType::LOADER_DATA)?;
        let (total, free) = Self::count(&map);
        let count = format!("{} MiB / {total} MiB", total - free);

        Ok(Self { count })
    }

    fn count(map: &MemoryMapOwned) -> (u64, u64) {
        let (total, free) =
            map.entries().fold((0, 0), |(total, free), entry| {
                match (
                    Self::is_total(entry.ty),
                    matches!(entry.ty, MemoryType::CONVENTIONAL),
                ) {
                    (true, true) => {
                        (total + entry.page_count, free + entry.page_count)
                    }
                    (true, false) => (total + entry.page_count, free),
                    _ => (total, free),
                }
            });
        let size = MIB / PAGE_SIZE as u64;

        (total / size, free / size)
    }

    const fn is_total(ty: MemoryType) -> bool {
        matches!(
            ty,
            MemoryType::CONVENTIONAL
                | MemoryType::RESERVED
                | MemoryType::LOADER_DATA
                | MemoryType::LOADER_CODE
                | MemoryType::BOOT_SERVICES_DATA
                | MemoryType::BOOT_SERVICES_CODE
                | MemoryType::RUNTIME_SERVICES_DATA
                | MemoryType::RUNTIME_SERVICES_CODE
                | MemoryType::PERSISTENT_MEMORY
                | MemoryType::ACPI_RECLAIM
                | MemoryType::ACPI_NON_VOLATILE
                | MemoryType::MMIO
                | MemoryType::MMIO_PORT_SPACE
        )
    }
}

impl InfoItem for Memory {
    fn render(&self) -> impl Iterator<Item = (&str, &str)> {
        [("Memory:", self.count.as_str())].into_iter()
    }
}

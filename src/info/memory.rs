use crate::{
    error::Result,
    info::{FromArgs, InfoItem},
};
use heapless::String;
use uefi::{
    Error, Status,
    boot::{MemoryType, PAGE_SIZE, memory_map},
    mem::memory_map::{MemoryMap, MemoryMapOwned},
};

const MB: u64 = 1024 * 1024;

#[derive(Clone)]
#[non_exhaustive]
pub struct Memory {
    pub memory: String<32>,
    pub physical_start: String<16>,
    pub virtual_start: String<16>,
}

impl Memory {
    pub fn new() -> Result<Self> {
        let map = memory_map(MemoryType::LOADER_DATA)?;
        let (total, usable, phys, virt) = Self::process_map(&map)?;

        let memory = String::build(format_args!("{usable} / {total} MiB"))?;
        let physical_start = String::build(format_args!("{phys:#x}"))?;
        let virtual_start = String::build(format_args!("{virt:#x}"))?;

        Ok(Self {
            memory,
            physical_start,
            virtual_start,
        })
    }

    fn process_map(map: &MemoryMapOwned) -> Result<(u64, u64, u64, u64)> {
        let mut total = 0u64;
        let mut usable = 0u64;
        let mut start_ptrs = None;

        for entry in map.entries() {
            let size = entry.page_count * PAGE_SIZE as u64;

            if Self::is_total(entry.ty) {
                total += size;
            }

            if entry.ty == MemoryType::CONVENTIONAL {
                usable += size;

                if start_ptrs.is_none() {
                    start_ptrs = Some((entry.phys_start, entry.virt_start));
                }
            }
        }

        let used = total - usable;
        let (phys, virt) = start_ptrs.ok_or(Error::new(
            Status::NOT_FOUND,
            "No pointers found from memmap",
        ))?;

        Ok((total / MB, used / MB, phys, virt))
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
        [
            ("Memory:", self.memory.as_str()),
            ("Physical start:", self.physical_start.as_str()),
            ("Virtual start:", self.virtual_start.as_str()),
        ]
        .into_iter()
    }
}

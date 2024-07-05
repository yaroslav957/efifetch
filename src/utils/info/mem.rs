use alloc::boxed::Box;
use uefi::data_types::{PhysicalAddress, VirtualAddress};
use uefi::prelude::BootServices;
use uefi::table::boot::{MemoryMap, MemoryType};

const MEMORY_LAYOUT_SIZE: usize = 256 * 64;

pub(crate) struct MemInfo {
    pub(crate) total_pages: u64,
    pub(crate) used_pages: u64,
    pub(crate) phys_addr: PhysicalAddress,
    pub(crate) virt_addr: VirtualAddress,
    pub(crate) memory_map: MemoryMap<'static>,
}

#[inline]
fn pages(memory_map: &MemoryMap) -> Option<(u64, u64)> {
    memory_map.entries().fold(Some((0u64, 0u64)), |acc, descriptor| {
        acc.map(|(total, used)| {
            (total + descriptor.page_count,
             if descriptor.ty == MemoryType::CONVENTIONAL {
                 used + descriptor.page_count
             } else {
                 used
             },)
        })
    })
}

impl MemInfo {
    pub(crate) fn get(boot_services: &BootServices) -> Self {
        let buf: &'static mut [u8] = Box::leak(Box::new([0u8; MEMORY_LAYOUT_SIZE]));
        let mut memory_map = boot_services.memory_map(buf)
            .expect("Cant get Memory map");
        let (total_pages, used_pages) = pages(&mut memory_map)
            .expect("Cant get Memory pages");

        Self {
            total_pages,
            used_pages,
            memory_map,
        }
    }
}

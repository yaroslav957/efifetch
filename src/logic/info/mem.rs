use uefi::{
    boot::MemoryType,
    data_types::{PhysicalAddress, VirtualAddress},
    mem::memory_map::{MemoryMap, MemoryMapOwned},
    Result,
};

#[allow(unused)]
#[derive(Debug)]
pub struct MemInfo {
    pub entries: usize,
    pub used_pages: u64,
    pub phys_addr: PhysicalAddress,
    pub virt_addr: VirtualAddress,
    pub map: MemoryMapOwned,
}

impl MemInfo {
    pub fn get() -> Result<Self> {
        let map = uefi::boot::memory_map(MemoryType::BOOT_SERVICES_DATA)?;
        let entries = map.meta().entry_count();
        let mut used_pages = 0;
        let mut phys_addr = PhysicalAddress::default();
        let mut virt_addr = VirtualAddress::default();

        for descriptor in map.entries() {
            used_pages += {
                match descriptor.ty {
                    MemoryType::CONVENTIONAL => descriptor.page_count,
                    _ => 0,
                }
            };

            phys_addr = descriptor.phys_start;
            virt_addr = descriptor.virt_start;
        }

        Ok(Self {
            used_pages,
            entries,
            phys_addr,
            virt_addr,
            map,
        })
    }
}

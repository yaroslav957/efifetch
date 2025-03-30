use uefi::{
    boot::MemoryType,
    data_types::{PhysicalAddress, VirtualAddress},
    mem::memory_map::{MemoryMap, MemoryMapOwned},
    Result,
};
#[allow(dead_code)]
#[derive(Debug)]
pub struct MemInfo {
    pub info: MemoryInfo,
    pub map: MemoryMapOwned,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MemoryInfo {
    pub pages: PagesInfo,
    pub entries: usize,
    pub phys_addr: PhysicalAddress,
    pub virt_addr: VirtualAddress,
}

#[derive(Default, Debug)]
pub struct PagesInfo {
    pub total: u64,
    pub used: u64,
}

impl MemInfo {
    pub fn get() -> Result<Self> {
        let map = uefi::boot::memory_map(MemoryType::BOOT_SERVICES_DATA)?;
        let info = MemoryInfo::from_map(&map)?;

        Ok(Self { info, map })
    }
}

impl MemoryInfo {
    fn from_map(map: &MemoryMapOwned) -> Result<Self> {
        let mut pages = PagesInfo::default();
        let mut phys_addr = PhysicalAddress::default();
        let mut virt_addr = VirtualAddress::default();
        let entries = map.meta().entry_count();

        for descriptor in map.entries() {
            pages.total += descriptor.page_count;
            pages.used += {
                match descriptor.ty {
                    MemoryType::CONVENTIONAL => descriptor.page_count,
                    _ => 0,
                }
            };

            phys_addr = descriptor.phys_start;
            virt_addr = descriptor.virt_start;
        }

        Ok(Self {
            pages,
            entries,
            phys_addr,
            virt_addr,
        })
    }
}

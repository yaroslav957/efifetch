use alloc::boxed::Box;

use uefi::data_types::{PhysicalAddress, VirtualAddress};
use uefi::prelude::BootServices;
use uefi::table::boot::{MemoryMap, MemoryType};

const MEMORY_LAYOUT_SIZE: usize = 4096 * 4;

#[derive(Default, Debug)]
pub(crate) struct PagesInfo {
    pub(crate) total: u64,
    pub(crate) used: u64,
}
#[derive(Debug)]
pub(crate) struct MemoryInfo {
    pub(crate) pages: PagesInfo,
    pub(crate) phys_addr: PhysicalAddress,
    pub(crate) virt_addr: VirtualAddress,
}

impl TryFrom<&MemoryMap<'_>> for MemoryInfo {
    type Error = ();

    fn try_from(map: &MemoryMap) -> Result<Self, Self::Error> {
        let mut pages = PagesInfo::default();
        let mut phys_addr = PhysicalAddress::default();
        let mut virt_addr = VirtualAddress::default();

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
            phys_addr,
            virt_addr,
        })
    }
}
#[allow(dead_code)]
pub(crate) struct MappedMemoryInfo {
    pub(crate) info: MemoryInfo,
    pub(crate) map: MemoryMap<'static>,
}

impl From<&BootServices> for MappedMemoryInfo {
    fn from(boot_services: &BootServices) -> Self {
        let buf: &'static mut [u8] = Box::leak(Box::new([0u8; MEMORY_LAYOUT_SIZE]));
        let map = boot_services.memory_map(buf).expect("Cant get Memory map");
        let info = MemoryInfo::try_from(&map).expect("Cant get Memory pages");

        Self { info, map }
    }
}

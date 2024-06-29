use raw_cpuid::{CpuId, CpuIdReaderNative, HypervisorInfo, ProcessorBrandString, VendorInfo};
use uefi::prelude::RuntimeServices;
use uefi::table::runtime::Time;

pub(crate) struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
}

pub(crate) struct CpuInfo {
    pub(crate) brand: ProcessorBrandString,
    pub(crate) vendor: VendorInfo,
    pub(crate) hypervisor: HypervisorInfo<CpuIdReaderNative>,
    pub(crate) vmx: bool,
    pub(crate) smx: bool,
}

impl From<Time> for Date {
    fn from(time: Time) -> Self {
        Self {
            day: time.day(),
            month: time.month(),
            year: time.year(),
        }
    }
}

impl Date {
    pub(crate) fn get(runtime_services: &RuntimeServices) -> Self {
        runtime_services.get_time()
            .unwrap().into()
    }
}

impl CpuInfo {
    pub(crate) fn get() -> Self {
        let cpu = CpuId::new();
        Self {
            brand: cpu.get_processor_brand_string()
                .expect("Cant get brand info"),
            vendor: cpu.get_vendor_info()
                .expect("Cant get vendor info"),
            hypervisor: cpu.get_hypervisor_info()
                .expect("Cant get hypervisor info"),
            vmx: cpu.get_feature_info()
                .expect("Cant get vmx info")
                .has_vmx(),
            smx: cpu.get_feature_info()
                .expect("Cant get smx info")
                .has_smx()
        }
    }
}
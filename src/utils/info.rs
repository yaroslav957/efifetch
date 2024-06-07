use core::fmt::Display;
use raw_cpuid::{CpuId, ProcessorBrandString, VendorInfo};
use uefi::prelude::RuntimeServices;
use uefi::print;
use uefi::table::runtime::Time;

pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

pub struct CpuInfo {
    pub brand: ProcessorBrandString,
    pub vendor: VendorInfo,
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
    pub fn get(runtime_services: &RuntimeServices) -> Self {
        runtime_services.get_time().unwrap().into()
    }
}

impl CpuInfo {
    pub fn get() -> Self {
        let cpu = CpuId::new();
        Self {
            brand: cpu.get_processor_brand_string()
                .expect("Cant get brand info"),
            vendor: cpu.get_vendor_info()
                .expect("Cant get vendor info"),
        }
    }
}

pub fn print_info<T: Display>(display: T, columns: usize) {
    print!("{:^width$}", display, width = columns)
}
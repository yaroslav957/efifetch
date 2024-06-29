use alloc::string::{String, ToString};
use core::arch::x86_64::{__cpuid, CpuidResult};

use uefi::prelude::RuntimeServices;
use uefi::table::runtime::Time;

const VMX_BITMASK: u32 = 1 << 5;
const SMX_BITMASK: u32 = 1 << 6;

pub(crate) struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
}

pub(crate) struct CpuInfo {
    pub(crate) brand: String,
    pub(crate) vendor: String,
    pub(crate) hypervisor: String,
    pub(crate) vmx: bool,
    pub(crate) smx: bool,
}


#[repr(u32)]
enum Leaf {
    Vendor = 0x0,
    Vmx = 0x1,
    Hypervisor = 0x40000000,
    Smx = 0x80000001,
    Brand = 0x80000002,
}

#[inline]
fn vmx_support() -> Option<bool> {
    let cpuid_result = unsafe { __cpuid(Leaf::Vmx as _) };
    Some(cpuid_result.ecx & VMX_BITMASK != 0)
}

#[inline]
fn smx_support() -> Option<bool> {
    let cpuid_result = unsafe { __cpuid(Leaf::Smx as _) };
    Some(cpuid_result.ecx & SMX_BITMASK != 0)
}

#[inline]
fn get_cpuid_info(buffer: &mut [u8; 12], cpuid: Leaf) {
    unsafe {
        let CpuidResult { ebx, ecx, edx, .. } = __cpuid(cpuid as _);
        let cpuid_result = [ebx, ecx, edx];
        for i in 0..3 {
            let name_slice = &mut buffer[4 * i..4 * (i + 1)];
            name_slice.copy_from_slice(&cpuid_result[i].to_le_bytes());
        }
    }
}

#[inline]
fn getinfo(buffer: &mut [u8; 48]) {
    unsafe {
        for i in 0..3_usize {
            let CpuidResult { ebx, ecx, edx, .. } = __cpuid(Leaf::Brand as u32 + i as u32);
            let cpuid_result = [ebx, ecx, edx];
            for j in 0..4 {
                let name_slice = &mut buffer[16 * i..][4 * j..][0..4];
                name_slice.copy_from_slice(&cpuid_result[j].to_le_bytes());
            }
        }
    }
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
        let mut hypervisor_buff = [0u8; 12];
        let mut vendor_buff = [0u8; 12];
        let mut brand_buff = [0u8; 48];
        get_cpuid_info(&mut hypervisor_buff, Leaf::Hypervisor);
        get_cpuid_info(&mut vendor_buff, Leaf::Vendor);
        getinfo(&mut brand_buff);

        Self {
            brand: core::str::from_utf8(&brand_buff)
                .expect("Cant get vendor info")
                .trim_matches('\0')
                .to_string(),
            vendor: core::str::from_utf8(&vendor_buff)
                .expect("Cant get vendor info")
                .to_string(),
            hypervisor: core::str::from_utf8(&hypervisor_buff)
                .expect("Cant get hypervisor info")
                .to_string(),
            vmx: vmx_support()
                .expect("Cant get vmx info"),
            smx: smx_support()
                .expect("Cant get smx info"),
        }
    }
}
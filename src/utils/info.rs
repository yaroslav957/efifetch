use alloc::string::{String, ToString};
use core::arch::x86_64::{__cpuid, CpuidResult};
use core::mem::transmute;

use uefi::prelude::RuntimeServices;
use uefi::table::runtime::Time;

pub(crate) struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
}

pub(crate) struct CpuInfo {
    pub(crate) brand: String,
    pub(crate) vendor: String,
    pub(crate) hypervisor: &'static str,
    pub(crate) vmx: bool,
    pub(crate) smx: bool,
}

const VMX_BITMASK: u32 = 1 << 5;
const SMX_BITMASK: u32 = 1 << 6;

#[repr(u32)]
#[derive(Clone, Copy)]
enum Leaf {
    Vendor = 0x0,
    Vmx = 0x1,
    Hypervisor = 0x40000000,
    Smx = 0x80000001,
    Brand = 0x80000002,
}

enum Hypervisor {
    KVM,
    VMware,
    HyperV,
    Qemu,
}

#[inline]
fn feature_support(cpuid: Leaf, bitmask: u32) -> Option<bool> {
    let cpuid_result = unsafe { __cpuid(cpuid as _) };
    Some(cpuid_result.ecx & bitmask != 0)
}

#[inline]
fn get_cpuid_info(buffer: &mut [u8; 12], cpuid: Leaf) {
    unsafe {
        let CpuidResult { ebx, ecx, edx, .. } = __cpuid(cpuid as _);
        let cpuid_result = match cpuid {
            Leaf::Hypervisor => [ebx, ecx, edx],
            Leaf::Vendor => [ebx, edx, ecx],
            _ => unreachable!("Invalid Leaf"),
        };

        for i in 0..3 {
            let name_slice = &mut buffer[4 * i..4 * (i + 1)];
            name_slice.copy_from_slice(&cpuid_result[i].to_le_bytes());
        }
    }
}

#[inline]
fn get_cpuid_brand(buffer: &mut [u8; 48]) {
    *buffer = unsafe {
        transmute::<[[u8; 16]; 3], _>(core::array::from_fn(|i| {
            let CpuidResult { eax, ebx, ecx, edx } = __cpuid(Leaf::Brand as u32 + i as u32);
            let cpuid_result = [eax, ebx, ecx, edx];
            transmute(cpuid_result.map(u32::to_le_bytes))
        }))
    };
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

impl TryFrom<[u8; 12]> for Hypervisor {
    type Error = ();

    fn try_from(cpuid_result: [u8; 12]) -> Result<Self, Self::Error> {
        match &cpuid_result {
            b"KVMKVMKVM\0\0\0" => Ok(Self::KVM),
            b"VMwareVMware" => Ok(Self::VMware),
            b"Microsoft Hv" => Ok(Self::HyperV),
            b"TCGTCGTCGTCG" => Ok(Self::Qemu),
            _ => Err(()),
        }
    }
}

impl Hypervisor {
    fn name(self) -> &'static str {
        match self {
            Self::KVM => "KVM",
            Self::VMware => "VMware",
            Self::HyperV => "HyperV",
            Self::Qemu => "Qemu",
        }
    }
}

impl CpuInfo {
    pub(crate) fn get() -> Self {
        let mut hypervisor_buff = [0u8; 12];
        let mut vendor_buff = [0u8; 12];
        let mut brand_buff = [0u8; 48];

        get_cpuid_info(&mut hypervisor_buff, Leaf::Hypervisor);
        get_cpuid_info(&mut vendor_buff, Leaf::Vendor);
        get_cpuid_brand(&mut brand_buff);

        Self {
            brand: core::str::from_utf8(&brand_buff)
                .expect("Cant get brand info")
                .trim_matches('\0')
                .to_string(),
            vendor: core::str::from_utf8(&vendor_buff)
                .expect("Cant get vendor info")
                .to_string(),
            hypervisor: Hypervisor::try_from(hypervisor_buff)
                .expect("Cant get Hypervisor info")
                .name(),
            vmx: feature_support(Leaf::Vmx, VMX_BITMASK)
                .expect("Cant get vmx info"),
            smx: feature_support(Leaf::Smx, SMX_BITMASK)
                .expect("Cant get vmx info"),
        }
    }
}
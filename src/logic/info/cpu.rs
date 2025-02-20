use alloc::string::{String, ToString};
use core::{
    arch::x86_64::{CpuidResult, __cpuid},
    mem::transmute,
};

const VMX_BITMASK: u32 = 1 << 5;
const SMX_BITMASK: u32 = 1 << 6;

pub struct CpuInfo {
    pub brand: String,
    pub vendor: String,
    pub hypervisor: &'static str,
    pub vmx: bool,
    pub smx: bool,
}

#[repr(u32)]
#[derive(Clone, Copy)]
enum Leaf {
    Vendor = 0x0,
    Vmx = 0x1,
    Hypervisor = 0x40000000,
    Smx = 0x80000001,
    Brand = 0x80000002,
}

#[derive(Default)]
enum Hypervisor {
    KVM,
    VMware,
    HyperV,
    Qemu,
    #[default]
    Unknown,
}

#[inline]
fn feature_support(cpuid: Leaf, bitmask: u32) -> bool {
    let cpuid_result = unsafe { __cpuid(cpuid as _) };
    cpuid_result.ecx & bitmask != 0
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

impl Hypervisor {
    fn new(cpuid_result: [u8; 12]) -> Self {
        match &cpuid_result {
            b"KVMKVMKVM\0\0\0" => Self::KVM,
            b"VMwareVMware" => Self::VMware,
            b"Microsoft Hv" => Self::HyperV,
            b"TCGTCGTCGTCG" => Self::Qemu,
            _ => Self::Unknown,
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
            Self::Unknown => "Unknown",
        }
    }
}

impl CpuInfo {
    pub fn get() -> Self {
        let mut hypervisor_buff = [0u8; 12];
        let mut vendor_buff = [0u8; 12];
        let mut brand_buff = [0u8; 48];

        get_cpuid_info(&mut hypervisor_buff, Leaf::Hypervisor);
        get_cpuid_info(&mut vendor_buff, Leaf::Vendor);
        get_cpuid_brand(&mut brand_buff);

        Self {
            brand: core::str::from_utf8(&brand_buff)
                .unwrap_or("Unknown")
                .trim_matches('\0')
                .to_string(),
            vendor: core::str::from_utf8(&vendor_buff)
                .unwrap_or("Unknown")
                .to_string(),
            hypervisor: Hypervisor::new(hypervisor_buff).name(),
            vmx: feature_support(Leaf::Vmx, VMX_BITMASK),
            smx: feature_support(Leaf::Smx, SMX_BITMASK),
        }
    }
}

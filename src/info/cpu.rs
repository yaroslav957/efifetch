use core::{
    arch::x86_64::{__cpuid, CpuidResult},
    mem::transmute,
};

const VMX: u32 = 1 << 5;
const SMX: u32 = 1 << 6;

#[repr(u32)]
enum Leaf {
    Vendor = 0x0,
    Vmx = 0x1,
    Hypervisor = 0x40000000,
    Smx = 0x80000001,
    Brand = 0x80000002,
}

pub struct Cpu {
    pub brand: &'static str,
    pub vendor: &'static str,
    pub hvisor: &'static str,
    pub vmx: bool,
    pub smx: bool,
}

fn feature_support(cpuid: Leaf, bitmask: u32) -> bool {
    let cpuid_result = unsafe { __cpuid(cpuid as _) };

    cpuid_result.ecx & bitmask != 0
}

fn cpuid_info(buffer: &mut [u8; 12], cpuid: Leaf) {
    unsafe {
        let CpuidResult { ebx, ecx, edx, .. } = __cpuid(&cpuid as _);
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

fn cpuid_brand(buffer: &mut [u8; 48]) {
    *buffer = unsafe {
        transmute::<[[u8; 16]; 3], _>(core::array::from_fn(|i| {
            let CpuidResult { eax, ebx, ecx, edx } = __cpuid(Leaf::Brand as u32 + i as u32);
            let cpuid_result = [eax, ebx, ecx, edx];
            transmute(cpuid_result.map(u32::to_ne_bytes))
        }))
    };
}

fn humanize_hypervisor(cpuid_result: [u8; 12]) -> &'static str {
    match &cpuid_result {
        b"KVMKVMKVM\0\0\0" => "KVM",
        b"VMwareVMware" => "VMware",
        b"Microsoft Hv" => "Microsoft Hv",
        b"TCGTCGTCGTCG" => "Qemu",
        _ => "Unknown",
    }
}

impl Cpu {
    pub fn new() -> Self {
        let mut hypervisor_buff = [0u8; 12];
        let mut vendor_buff = [0u8; 12];
        let mut brand_buff = [0u8; 48];

        cpuid_info(&mut hypervisor_buff, Leaf::Hypervisor);
        cpuid_info(&mut vendor_buff, Leaf::Vendor);
        cpuid_brand(&mut brand_buff);

        Self {
            brand: core::str::from_utf8(&brand_buff)
                .unwrap_or("Unknown")
                .trim_matches('\0'),
            vendor: core::str::from_utf8(&vendor_buff).unwrap_or("Unknown"),
            hvisor: humanize_hypervisor(hypervisor_buff),
            vmx: feature_support(Leaf::Vmx, VMX),
            smx: feature_support(Leaf::Smx, SMX),
        }
    }
}

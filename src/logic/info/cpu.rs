use core::{
    arch::x86_64::{__cpuid, CpuidResult},
    mem::transmute,
};

const VMX_BITMASK: u32 = 1 << 5;
const SMX_BITMASK: u32 = 1 << 6;

// wait for 0.2.0 release *
#[allow(dead_code)] // <--*
pub struct CpuInfo {
    pub brand: Brand,
    pub vendor: Vendor,
    pub hypervisor: Hypervisor,
    pub vmx: bool,
    pub smx: bool,
}

#[repr(transparent)]
pub struct Brand([u8; 48]);

#[repr(transparent)]
pub struct Vendor([u8; 12]);

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
pub enum Hypervisor {
    KVM,
    VMware,
    HyperV,
    Qemu,
    #[default]
    Unknown,
}

fn feature_support(cpuid: Leaf, bitmask: u32) -> bool {
    let cpuid_result = unsafe { __cpuid(cpuid as _) };
    cpuid_result.ecx & bitmask != 0
}

fn get_cpuid_info(cpuid: Leaf) -> [u8; 12] {
    let mut buf = [0u8; 12];
    unsafe {
        let CpuidResult { ebx, ecx, edx, .. } = __cpuid(cpuid as _);
        let cpuid_result = match cpuid {
            Leaf::Hypervisor => [ebx, ecx, edx],
            Leaf::Vendor => [ebx, edx, ecx],
            _ => unreachable!("Invalid Leaf"),
        };

        for i in 0..3 {
            let name_slice = &mut buf[4 * i..4 * (i + 1)];
            name_slice.copy_from_slice(&cpuid_result[i].to_le_bytes());
        }

        buf
    }
}

#[allow(dead_code)] // <--*
impl Brand {
    #[allow(unused_assignments)]
    fn get_cpuid_brand() -> Self {
        let mut buf = [0u8; 48];
        buf = unsafe {
            transmute::<[[u8; 16]; 3], _>(core::array::from_fn(|i| {
                let CpuidResult { eax, ebx, ecx, edx } = __cpuid(Leaf::Brand as u32 + i as u32);
                let cpuid_result = [eax, ebx, ecx, edx];
                transmute(cpuid_result.map(u32::to_le_bytes))
            }))
        };

        Self(buf)
    }
    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.0)
            .unwrap_or("Unknown Brand")
            .trim_matches('\0')
    }
}

#[allow(dead_code)] // <--*
impl Vendor {
    fn new(cpuid_result: [u8; 12]) -> Self {
        Self(cpuid_result)
    }

    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.0).unwrap_or("Unknown Vendor")
    }
}

#[allow(dead_code)] // <--*
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

    pub fn name(self) -> &'static str {
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
        let vmx = feature_support(Leaf::Vmx, VMX_BITMASK);
        let smx = feature_support(Leaf::Smx, SMX_BITMASK);
        let hypervisor = Hypervisor::new(get_cpuid_info(Leaf::Hypervisor));
        let vendor = Vendor::new(get_cpuid_info(Leaf::Vendor));
        let brand = Brand::get_cpuid_brand();

        Self {
            brand,
            vendor,
            hypervisor,
            vmx,
            smx,
        }
    }
}

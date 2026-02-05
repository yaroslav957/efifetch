use crate::consts::firmware_vendors::*;
use uefi::{
    CStr16,
    system::{firmware_revision, firmware_vendor, uefi_revision},
};

#[derive(Clone, Copy)]
pub struct Firmware {
    revision: u32,
    vendor: &'static CStr16,
    uefi_revision: u32,
}

impl Firmware {
    pub fn new() -> Self {
        let revision = firmware_revision();
        let vendor = firmware_vendor();
        let uefi_revision = uefi_revision().0;

        Self {
            revision,
            vendor,
            uefi_revision,
        }
    }

    pub fn revision(&self) -> u32 {
        self.revision
    }

    pub fn vendor(&self) -> &'static str {
        match self.vendor.as_bytes() {
            AMI_VENDOR => "Aptio V",
            INSYDE_VENDOR => "InsydeH2O",
            TIANOCORE_VENDOR => "EDK II",
            _ => "Unknown",
        }
    }

    pub fn uefi_revision(&self) -> u32 {
        self.uefi_revision
    }
}

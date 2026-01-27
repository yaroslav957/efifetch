use crate::{consts::firmware_vendors::*, utils::U32Buffer};
use uefi::{
    CStr16,
    system::{firmware_revision, firmware_vendor, uefi_revision},
};

#[derive(Clone, Copy)]
pub struct Firmware {
    revision: U32Buffer,
    vendor: &'static CStr16,
    uefi_revision: U32Buffer,
}

impl Firmware {
    pub fn new() -> Self {
        let revision = U32Buffer::new(firmware_revision());
        let vendor = firmware_vendor();
        let uefi_revision = U32Buffer::new(uefi_revision().0);

        Self {
            revision,
            vendor,
            uefi_revision,
        }
    }

    pub fn revision(&self) -> U32Buffer {
        self.revision
    }

    pub fn vendor(&self) -> &'static str {
        match self.vendor.as_bytes() {
            EDKII_VENDOR => "EDK II",
            INSYDE_VENDOR => "InsydeH2O",
            _ => "Unknown",
        }
    }

    pub fn uefi_revision(&self) -> U32Buffer {
        self.uefi_revision
    }
}

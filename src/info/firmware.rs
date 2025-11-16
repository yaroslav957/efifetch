use crate::{consts::EDK_VENDOR, info::U32Buffer};
use uefi::{
    CStr16,
    system::{firmware_revision, firmware_vendor, uefi_revision},
};

#[allow(dead_code)]
pub struct Firmware {
    pub revision: U32Buffer,
    pub vendor: &'static str,
    pub uefi_revision: U32Buffer,
}

impl Firmware {
    pub fn new() -> Self {
        let revision = U32Buffer::new(firmware_revision());
        let vendor = Firmware::vendor(firmware_vendor());
        let uefi_revision = U32Buffer::new(uefi_revision().0);

        Self {
            revision,
            vendor,
            uefi_revision,
        }
    }

    fn vendor(cstr: &'static CStr16) -> &'static str {
        match cstr.as_bytes() {
            EDK_VENDOR => "EDK II",
            _ => "Unknown",
        }
    }
}

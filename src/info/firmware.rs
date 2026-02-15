use crate::error::Result;
use heapless::String;
use uefi::system::{firmware_revision, firmware_vendor, uefi_revision};

// Based on UEFI max path (255 UTF-16 symbols),
// 3x expansion factor for UTF-8 conversion from UCS-2,
// 1 byte for null-terminator compatibility
const SIZE: usize = 255 * 3 + 1;

#[derive(Clone)]
pub struct Firmware {
    revision: u32,
    vendor: String<SIZE>,
    uefi_revision: u32,
}

impl Firmware {
    pub fn new() -> Result<Self> {
        let revision = firmware_revision();
        let vendor = firmware_vendor();
        let uefi_revision = uefi_revision().0;

        let buf = vendor.to_u16_slice();
        let vendor = String::from_utf16(buf)?;

        Ok(Self {
            revision,
            vendor,
            uefi_revision,
        })
    }

    pub fn revision(&self) -> u32 {
        self.revision
    }

    pub fn uefi_revision(&self) -> u32 {
        self.uefi_revision
    }
}

use crate::{error::Result, info::InfoItem};
use core::fmt::Write;
use heapless::String;
use uefi::system::{firmware_revision, firmware_vendor, uefi_revision};

// Based on UEFI max path (255 UTF-16 symbols),
// 3x expansion factor for UTF-8 conversion from UCS-2,
// 1 byte for null-terminator compatibility
const SIZE: usize = 255 * 3 + 1;

#[derive(Clone)]
pub struct Firmware {
    pub revision: String<10>,
    pub vendor: String<SIZE>,
    pub uefi_revision: String<10>,
}

impl Firmware {
    pub fn new() -> Result<Self> {
        let revision = {
            let mut s = String::new();
            write!(&mut s, "{}", firmware_revision())?;
            s
        };

        let vendor = {
            let buf = firmware_vendor().to_u16_slice();
            String::from_utf16(buf)?
        };

        let uefi_revision = {
            let mut s = String::new();
            write!(&mut s, "{}", uefi_revision().0)?;
            s
        };

        Ok(Self {
            revision,
            vendor,
            uefi_revision,
        })
    }
}

impl InfoItem for Firmware {
    fn render(&self) -> impl Iterator<Item = (&str, &str)> {
        [
            ("Rebision:", self.revision.as_str()),
            ("Vendor:", self.vendor.as_str()),
            ("Uefi revision:", self.uefi_revision.as_str()),
        ]
        .into_iter()
    }
}

use crate::{
    error::Result,
    info::{FromArgs, InfoItem},
};

use alloc::string::String;

use uefi::system::{firmware_revision, firmware_vendor, uefi_revision};

#[derive(Clone)]
#[non_exhaustive]
pub struct Firmware {
    pub revision: String,
    pub vendor: String,
    pub uefi_revision: String,
}

impl Firmware {
    pub fn new() -> Result<Self> {
        let revision = String::build(format_args!("{}", firmware_revision()))?;
        let vendor = {
            let buf = firmware_vendor().to_u16_slice();
            String::from_utf16(buf)?
        };
        let uefi_revision = String::build(format_args!(
            "{}.{}",
            uefi_revision().major(),
            uefi_revision().minor()
        ))?;

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
            ("Revision:", self.revision.as_str()),
            ("Vendor:", self.vendor.as_str()),
            ("UEFI revision:", self.uefi_revision.as_str()),
        ]
        .into_iter()
    }
}

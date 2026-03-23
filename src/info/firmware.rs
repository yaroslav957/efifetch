use crate::{error::Result, info::InfoItem};

use alloc::{format, string::String};
use core::str::from_utf8;

use uefi::{
    Error, Status, cstr16,
    runtime::{VariableVendor, get_time, get_variable_boxed},
    system::{firmware_vendor, uefi_revision},
};

const DISABLED: u8 = 0x0;
const ENABLED: u8 = 0x1;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Firmware {
    pub date: String,
    pub time: String,
    pub vendor: String,
    pub secure_boot: &'static str,
    pub platform_lang: String,
}

impl Firmware {
    pub fn new() -> Result<Self> {
        let (date, time) = Self::time()?;
        let vendor = {
            let buf = firmware_vendor().to_u16_slice();

            format!(
                "{} {}.{}",
                String::from_utf16(buf)?,
                uefi_revision().major(),
                uefi_revision().minor()
            )
        };
        let secure_boot = {
            let (buf, _) = get_variable_boxed(
                cstr16!("SecureBoot"),
                &VariableVendor::GLOBAL_VARIABLE,
            )?;

            match &*buf {
                [DISABLED] => "Disabled",
                [ENABLED] => "Enabled",
                _ => "Unknown/Unsupported",
            }
        };
        let platform_lang = {
            let (buf, _) = get_variable_boxed(
                cstr16!("PlatformLang"),
                &VariableVendor::GLOBAL_VARIABLE,
            )?;
            let bytes = buf.split(|&b| b == 0).next().ok_or(Error::new(
                Status::UNSUPPORTED,
                "Failed to split PlatformLang result",
            ))?;

            format!("{}.UTF-8", from_utf8(bytes)?)
        };

        Ok(Self {
            time,
            date,
            vendor,
            secure_boot,
            platform_lang,
        })
    }

    fn time() -> Result<(String, String)> {
        let time = get_time()?;
        Ok((
            format!("{:02}/{:02}/{}", time.day(), time.month(), time.year()),
            format!("{:02}:{:02} (UTC+3:00)", time.hour() + 3, time.minute()),
        ))
    }
}

impl InfoItem for Firmware {
    fn render(&self) -> impl Iterator<Item = (&str, &str)> {
        [
            ("Date:", self.date.as_str()),
            ("Time:", self.time.as_str()),
            ("Vendor:", self.vendor.as_str()),
            ("Secure Boot:", self.secure_boot),
            ("Language:", self.platform_lang.as_str()),
        ]
        .into_iter()
    }
}

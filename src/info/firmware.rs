use crate::{error::Result, info::InfoItem};

use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
};
use uefi::{
    CStr16, cstr16,
    runtime::{VariableVendor, get_time, get_variable_boxed},
    system::{firmware_vendor, uefi_revision},
};

const DISABLED: u8 = 0x0;
const ENABLED: u8 = 0x1;

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Firmware {
    pub timestamp: Timestamp,
    pub vendor: String,
    pub secure_boot: &'static str,
    pub platform_lang: &'static str,
}

#[derive(Clone, Debug, Default)]
pub struct Timestamp {
    pub date: String,
    pub time: String,
}

impl Firmware {
    pub fn new() -> Result<Self> {
        let timestamp = Timestamp::new()?;
        let vendor = format!(
            "{} {}.{}",
            firmware_vendor().to_string(),
            uefi_revision().major(),
            uefi_revision().minor()
        );
        let secure_boot = match &*Self::global_variable(cstr16!("SecureBoot"))?
        {
            [DISABLED] => "Disabled",
            [ENABLED] => "Enabled",
            _ => "Unknown/Unsupported",
        };
        let platform_lang =
            match &*Self::global_variable(cstr16!("PlatformLang"))? {
                [b'e', b'n', ..] => "en_US.UTF-8",
                [b'r', b'u', ..] => "ru_RU.UTF-8",
                _ => "Unknown.UTF-8",
            };

        Ok(Self {
            timestamp,
            vendor,
            secure_boot,
            platform_lang,
        })
    }

    fn global_variable(name: &CStr16) -> Result<Box<[u8]>> {
        let (buf, _) =
            get_variable_boxed(name, &VariableVendor::GLOBAL_VARIABLE)?;

        Ok(buf)
    }
}

impl InfoItem for Firmware {
    fn render(&self) -> impl Iterator<Item = (&str, &str)> {
        [
            ("Date:", self.timestamp.date.as_str()),
            ("Time:", self.timestamp.time.as_str()),
            ("Vendor:", self.vendor.as_str()),
            ("Secure Boot:", self.secure_boot),
            ("Language:", self.platform_lang),
        ]
        .into_iter()
    }
}

impl Timestamp {
    pub fn new() -> Result<Self> {
        let time = get_time()?;

        let (date, time) = (
            format!("{:02}/{:02}/{}", time.day(), time.month(), time.year()),
            format!("{:02}:{:02} (UTC+3:00)", time.hour() + 3, time.minute()),
        );

        Ok(Self { time, date })
    }
}

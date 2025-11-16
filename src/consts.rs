use uefi::Char16;

pub const MSRV: &str = env!("CARGO_PKG_RUST_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub const LICENSE: &str = env!("CARGO_PKG_LICENSE");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub const KEY_M: Char16 = unsafe { Char16::from_u16_unchecked(0x006D) };
pub const KEY_A: Char16 = unsafe { Char16::from_u16_unchecked(0x0061) };
pub const KEY_E: Char16 = unsafe { Char16::from_u16_unchecked(0x0065) };

pub const EDK_VENDOR: &[u8] = b"E\0D\0K\0 \0I\0I\0\0\0";

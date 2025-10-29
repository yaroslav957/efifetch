use uefi::Char16;

pub const _UCS2_SURROGATE_START: u16 = 0xD800;
pub const _UCS2_SURROGATE_END: u16 = 0xDFFF;
pub const KEY_M: Char16 = unsafe { Char16::from_u16_unchecked(0x006D) };
pub const KEY_A: Char16 = unsafe { Char16::from_u16_unchecked(0x0061) };
pub const KEY_E: Char16 = unsafe { Char16::from_u16_unchecked(0x0065) };

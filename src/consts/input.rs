use uefi::Char16;

pub const C_KEY: Char16 = unsafe { Char16::from_u16_unchecked(0x0063) };
pub const E_KEY: Char16 = unsafe { Char16::from_u16_unchecked(0x0065) };
pub const F_KEY: Char16 = unsafe { Char16::from_u16_unchecked(0x0066) };
pub const M_KEY: Char16 = unsafe { Char16::from_u16_unchecked(0x006D) };
pub const P_KEY: Char16 = unsafe { Char16::from_u16_unchecked(0x0070) };
pub const R_KEY: Char16 = unsafe { Char16::from_u16_unchecked(0x0072) };

use crate::Out;
use uefi::{Char16, Error, Result, Status};

const MIN_CONSOLE_WIDTH: usize = 80;
const MIN_CONSOLE_HEIGHT: usize = 25;

const UCS2_SURROGATE_START: u16 = 0xD800;
const UCS2_SURROGATE_END: u16 = 0xDFFF;

pub const fn char16(c: u16) -> Option<Char16> {
    if c >= UCS2_SURROGATE_START && c <= UCS2_SURROGATE_END {
        None
    } else {
        unsafe { Some(Char16::from_u16_unchecked(c)) }
    }
}

pub fn resolution(out: &Out) -> Result<[usize; 2]> {
    let mode = out.current_mode()?.unwrap();

    Ok([mode.columns(), mode.rows()])
}

pub fn check_resolution(width: usize, height: usize) -> Result<Status> {
    if width < MIN_CONSOLE_WIDTH || height < MIN_CONSOLE_HEIGHT {
        Err(Error::new(Status::UNSUPPORTED, ()))
    } else {
        Ok(Status::SUCCESS)
    }
}

pub fn minimize(out: &mut Out) -> Result<()> {
    let min_mode = out.modes().min().unwrap();

    out.set_mode(min_mode)
}

pub fn digits_count(num: u64) -> usize {
    if num.eq(&0) {
        1
    } else {
        num.ilog10() as usize + 1
    }
}

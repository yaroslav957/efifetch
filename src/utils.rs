use crate::Out;
use uefi::{Error, Result, Status};

const MIN_CONSOLE_WIDTH: usize = 80;
const MIN_CONSOLE_HEIGHT: usize = 25;

pub fn resolution(out: &Out) -> Result<[usize; 2]> {
    let mode = out.current_mode()?.unwrap();

    Ok([mode.columns(), mode.rows()])
}

pub fn check_resolution(width: usize, height: usize) -> Result<Status> {
    if width < MIN_CONSOLE_WIDTH || height < MIN_CONSOLE_HEIGHT {
        return Err(Error::new(Status::UNSUPPORTED, ()));
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

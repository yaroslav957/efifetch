use crate::Out;
use uefi::Result;

pub fn resolution(out: &Out) -> Result<[usize; 2]> {
    let mode = out.current_mode()?.unwrap();

    Ok([mode.columns(), mode.rows()])
}

#[cold]
pub fn minimize(out: &mut Out) -> Result<()> {
    let min_mode = out.modes().min().unwrap();
    let mi_mode = out.modes().last().unwrap();

    out.set_mode(min_mode)
}

pub fn digit_count(mut num: u64) -> usize {
    if num == 0 {
        return 1;
    }

    let mut count = 0;

    while num > 0 {
        num /= 10;
        count += 1;
    }

    count
}

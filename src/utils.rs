use crate::Out;
use uefi::Result;

pub fn resolution(out: &Out) -> Result<[usize; 2]> {
    let mode = out.current_mode()?.unwrap();
    Ok([mode.columns(), mode.rows()])
}

#[cold]
pub fn minimize(out: &mut Out) -> Result<()> {
    let min_mode = out.modes().min().unwrap();
    out.set_mode(min_mode)
}

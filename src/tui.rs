#![allow(unused)]

mod canvas;
pub use canvas::*;

use uefi::{Result, Status};

pub fn on_draw<F>(canvas: F) -> Result<()>
where
    F: FnMut(&mut Canvas) -> Result<()>,
{
    Ok(())
}

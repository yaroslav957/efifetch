//! TODO: Rewrite Shell args parsing with my own tiny cli-args parser lib,
//! called `Tenu` (https://github.com/yaroslav957/tenu).
//! The crate has no dependencies on alloc and none are planned.

#![no_std]
#![no_main]

mod consts;
mod error;
mod info;
mod output;

use crate::{error::Result, info::Info};
use uefi::{
    Status,
    boot::{get_handle_for_protocol, image_handle, open_protocol_exclusive},
    entry,
    proto::{console::text::Output, shell_params::ShellParameters},
};

#[entry]
fn main() -> Status {
    if let Err(e) = run() {
        return e.status();
    };

    Status::SUCCESS
}

#[allow(unused)]
pub fn run() -> Result<()> {
    // Handles
    let ih = image_handle();
    let oh = get_handle_for_protocol::<Output>()?;

    // Protocols
    let params = open_protocol_exclusive::<ShellParameters>(ih)?;
    let stdout = open_protocol_exclusive::<Output>(oh)?;

    // Crate structs
    let info = Info::new()?;

    /*
    let theme = Theme::default()

    match params.args() { ... }
    output::print(&mut stdout, info, theme, page)?;
    */

    for arg in params.args() {
        uefi::println!("{arg}");
    }

    Ok(())
}

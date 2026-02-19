//! TODO: Rewrite Shell args parsing with my own tiny cli-args parser lib,
//! called `Tenu` (https://github.com/yaroslav957/tenu).
//! The crate has no dependencies on alloc and none are planned.

#![no_std]
#![no_main]

mod error;
mod info;
mod output;

use crate::{
    error::Result,
    info::Info,
    output::{draw, theme::Theme},
};
use core::fmt::Write;
use heapless::{String, Vec};
use uefi::{
    CStr16, Status,
    boot::{get_handle_for_protocol, image_handle, open_protocol_exclusive},
    entry,
    proto::{console::text::Output, shell_params::ShellParameters},
};

const HELP: &str = r"usage: efifetch [options]
  options:
    -h, --help  Print help
    -l, --logo  Print info with uefi/vendor logo
";

#[derive(Default)]
struct Flags {
    help: bool,
    logo: bool,
}

#[entry]
fn main() -> Status {
    if let Err(e) = run() {
        return e.status();
    };

    Status::SUCCESS
}

fn run() -> Result<()> {
    // Handles
    let ih = image_handle();
    let oh = get_handle_for_protocol::<Output>()?;

    // Protocols
    let params = open_protocol_exclusive::<ShellParameters>(ih)?;
    let mut stdout = open_protocol_exclusive::<Output>(oh)?;

    //TODO:
    let mut args: Vec<String<64>, 16> = Vec::new();
    let mut flags = Flags::default();

    let info = Info::new()?;
    let theme = Theme::RED;

    convert(params.args(), &mut args)?;
    parse(args, &mut flags)?;

    if flags.help {
        writeln!(stdout, "{HELP}")?;

        return Ok(());
    }

    draw(&mut stdout, info, theme, flags.logo)?;

    Ok(())
}

fn convert<'c, I>(args: I, vec: &mut Vec<String<64>, 16>) -> Result<()>
where
    I: Iterator<Item = &'c CStr16>,
{
    for arg in args.skip(1) {
        let buf = arg.to_u16_slice();
        let string = String::from_utf16(buf)?;

        _ = vec.push(string)
    }

    Ok(())
}

fn parse(args: Vec<String<64>, 16>, flags: &mut Flags) -> Result<()> {
    for arg in args.iter() {
        match arg.as_str() {
            "-h" | "--help" => flags.help = true,
            "-l" | "--logo" => flags.logo = true,
            _ => flags.help = true,
        }
    }

    Ok(())
}

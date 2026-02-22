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
  options(TODO):
    -t=VALUE, --theme=VALUE
    -p=VALUE, --page=VALUE
";

#[derive(Clone, Copy, Default)]
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
    // Handles: The ImageHandle (ih)
    // represents the executable itself and is retrieved globally.
    // The Output protocol handle (oh)
    // must be explicitly looked up to find the specific
    // device handle that supports the console output interface.
    let ih = image_handle();
    let oh = get_handle_for_protocol::<Output>()?;

    // Protocols: exclusive mode to ensure we have direct control over
    // the shell state, preventing other EFI drivers from interfering
    // with our output or argument reading during execution.
    let params = open_protocol_exclusive::<ShellParameters>(ih)?;
    let mut stdout = open_protocol_exclusive::<Output>(oh)?;

    // UEFI spec doesn't strictly define max argument lengths,
    // this ~1KB stack allocation is a safe middle ground.
    // Don't care about the hard limit here because any overflow
    // is caught by Error::Capacity conversion
    let mut args: Vec<String<64>, 16> = Vec::new();
    let mut flags = Flags::default();

    convert(params.args(), &mut args)?;
    parse(args, &mut flags)?;

    // TODO:
    // match flags.theme {
    // Themes::Red => theme = Theme::RED,
    // Themes::Green => theme = Theme::GREEN,
    // ... => ...
    // _ => theme = Theme::default(),
    // };
    //
    let info = Info::new()?;
    let theme = Theme::RED;

    if flags.help {
        writeln!(stdout, "{HELP}")?;

        return Ok(());
    }

    draw(&mut stdout, info, theme, flags)?;

    Ok(())
}

fn convert<'c, I, const L: usize, const N: usize>(
    args: I,
    vec: &mut Vec<String<L>, N>,
) -> Result<()>
where
    I: Iterator<Item = &'c CStr16>,
{
    for arg in args.skip(1) {
        let buf = arg.to_u16_slice();
        let string = String::from_utf16(buf)?;

        _ = vec.push(string);
    }

    Ok(())
}

fn parse<const L: usize, const N: usize>(
    args: Vec<String<L>, N>,
    flags: &mut Flags,
) -> Result<()> {
    for arg in args.iter() {
        match arg.as_str() {
            "-h" | "--help" => flags.help = true,
            "-l" | "--logo" => flags.logo = true,
            _ => flags.help = true,
        }
    }

    Ok(())
}

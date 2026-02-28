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
    output::{draw, page::Page, theme::Theme},
};
use core::fmt::Write;
use heapless::{CapacityError, String, Vec};
use uefi::{
    CStr16, Status,
    boot::{
        ScopedProtocol, get_handle_for_protocol, image_handle,
        open_protocol_exclusive,
    },
    entry,
    proto::{console::text::Output, shell_params::ShellParameters},
};

const HELP: &str = r"usage: efifetch [options]
  options:
    -h, --help  Print help
    -l, --logo  Print info with uefi/vendor logo
  options(TODO):
    -p=VALUE, --page=VALUE
";

#[entry]
fn main() -> Status {
    if let Err(e) = run() {
        return e.status();
    };

    Status::SUCCESS
}

fn run() -> Result<()> {
    let ih = image_handle();
    let oh = get_handle_for_protocol::<Output>()?;

    let params = open_protocol_exclusive::<ShellParameters>(ih)?;
    let mut stdout = open_protocol_exclusive::<Output>(oh)?;

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
        flags.print_err(&mut stdout)?;
        writeln!(&mut stdout, "{HELP}")?;

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
        let s = String::from_utf16(buf)?;

        vec.push(s).map_err(|_| CapacityError::default())?;
    }

    Ok(())
}

fn parse<const L: usize, const N: usize>(
    args: Vec<String<L>, N>,
    flags: &mut Flags,
) -> Result<()> {
    for arg in args.iter() {
        let arg = arg.as_str();

        if let Some(val) = arg
            .strip_prefix("-p=")
            .or_else(|| arg.strip_prefix("--page="))
        {
            flags.page = match val {
                "main" | "MAIN" => Page::Main,
                "env" | "ENV" => Page::Env,
                _ => {
                    flags.help = true;
                    flags.invalid_page = true;
                    flags.page
                }
            };

            continue;
        }

        match arg {
            "-h" | "--help" => flags.help = true,
            "-l" | "--logo" => flags.logo = true,
            _ => {
                flags.help = true;
                flags.invalid_flag = true
            }
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Default)]
struct Flags {
    pub help: bool,
    pub invalid_flag: bool,
    pub invalid_page: bool,
    pub logo: bool,
    pub page: Page,
}

impl Flags {
    fn print_err(
        &self,
        stdout: &mut ScopedProtocol<Output>,
    ) -> core::fmt::Result {
        if self.invalid_flag {
            writeln!(
                stdout,
                "Invalid flag or command. Help for a list of available flags:\n"
            )?;
        }

        if self.invalid_page {
            writeln!(
                stdout,
                "Invalid page value. Help for a list of available pages:\n"
            )?;
        }

        Ok(())
    }
}

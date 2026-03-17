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
    flags:
        -h/--help: Print help
        -l/--logo: Print info with logo
        -v/--version: Print version
    options:
        -p/--page=VALUE,
            VELUE=[main, env, firm, mem]
        -t/--page=VALUE,
            VALUE=[red, green]
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

    let info = Info::new()?;
    let mut args: Vec<String<32>, 16> = Vec::new();
    let mut flags = Flags::default();

    convert(params.args(), &mut args)?;
    parse(args, &mut flags)?;

    if flags.help {
        flags.print_err(&mut stdout)?;
        writeln!(&mut stdout, "{HELP}")?;

        return Ok(());
    } else if flags.version {
        writeln!(
            &mut stdout,
            "{} version: {}",
            &info.env.name, &info.env.version
        )?;

        return Ok(());
    }

    draw(&mut stdout, info, flags)?;

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

/// Rewrite later with `Tenu` crate & delete this block
fn parse<const L: usize, const N: usize>(
    args: Vec<String<L>, N>,
    flags: &mut Flags,
) -> Result<()> {
    for arg in args.iter().map(|s| s.as_str()) {
        if let Some(val) = arg
            .strip_prefix("-p=")
            .or_else(|| arg.strip_prefix("--page="))
        {
            flags.page = match val {
                "main" | "MAIN" => Page::Main,
                "env" | "ENV" => Page::Env,
                "firm" | "FIRM" => Page::Firmware,
                "mem" | "MME" => Page::Memory,
                _ => {
                    flags.help = true;
                    flags.invalid_option = true;
                    flags.page
                }
            };

            continue;
        }

        if let Some(val) = arg
            .strip_prefix("-t=")
            .or_else(|| arg.strip_prefix("--theme="))
        {
            flags.theme = match val {
                "red" | "RED" => Theme::RED,
                "green" | "GREEN" => Theme::GREEN,
                _ => {
                    flags.help = true;
                    flags.invalid_option = true;
                    flags.theme
                }
            };

            continue;
        }

        match arg {
            "-h" | "--help" => flags.help = true,
            "-v" | "--version" => flags.version = true,
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
    pub version: bool,
    pub logo: bool,
    pub page: Page,
    pub theme: Theme,
    pub invalid_flag: bool,
    pub invalid_option: bool,
}

impl Flags {
    fn print_err(
        &self,
        stdout: &mut ScopedProtocol<Output>,
    ) -> core::fmt::Result {
        if self.invalid_flag {
            writeln!(
                stdout,
                "Invalid flag or command. List of available flags:\n"
            )?;
        }

        if self.invalid_option {
            writeln!(
                stdout,
                "Invalid option value. List of available options:\n"
            )?;
        }

        Ok(())
    }
}

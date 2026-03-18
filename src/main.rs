//! TODO: Rewrite Shell args parsing with my own tiny cli-args parser lib,
//! called `Tenu` (https://github.com/yaroslav957/tenu).

#![no_std]
#![no_main]

extern crate alloc;

use crate::{
    error::Result,
    info::Info,
    output::{draw, page::Page, theme::Theme},
};

use alloc::{string::String, vec::Vec};
use core::fmt::{self, Write};

use uefi::{
    CStr16, Status,
    boot::{
        ScopedProtocol, get_handle_for_protocol, image_handle,
        open_protocol_exclusive,
    },
    entry,
    helpers::init,
    proto::{console::text::Output, shell_params::ShellParameters},
};

mod error;
mod info;
mod output;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");
const RUSTC: &str = env!("VERGEN_RUSTC_SEMVER");

const HELP: &str = concat!(
    "Usage: efifetch [OPTION]...\n",
    "A simple system information tool for EFI environments.\n",
    "\n",
    "Options:\n",
    "  -h, --help           display this help and exit\n",
    "  -v, --version        output version information and exit\n",
    "  -l, --logo           print system information alongside the logo\n",
    "  -p, --page=PAGE      specify the information page to display\n",
    "                         (main, firm, mem)\n",
    "  -t, --theme=COLOR    set the output color theme\n",
    "                         (red, green)\n",
    "\n",
    "Examples:\n",
    "  efifetch -l -p=mem   Show memory info with a logo\n",
    "  efifetch --theme=red Set theme to red\n",
    "\n",
    "Report bugs to <https://github.com/yaroslav957/efifetch>"
);

#[entry]
fn main() -> Status {
    if let Err(e) = run() {
        return e.status();
    };

    Status::SUCCESS
}

fn run() -> Result<()> {
    init()?;

    let ih = image_handle();
    let oh = get_handle_for_protocol::<Output>()?;

    let params = open_protocol_exclusive::<ShellParameters>(ih)?;
    let mut stdout = open_protocol_exclusive::<Output>(oh)?;

    let info = Info::new()?;
    let args = convert(params.args())?;
    let mut flags = Flags::default();

    parse(&args, &mut flags)?;

    if flags.help {
        flags.print_err(&mut stdout)?;
        writeln!(&mut stdout, "{HELP}")?;

        return Ok(());
    } else if flags.version {
        writeln!(&mut stdout, "Version: {VERSION} builded on Rust {RUSTC}")?;
        writeln!(&mut stdout, "Time: {TIMESTAMP}")?;

        return Ok(());
    }

    draw(&mut stdout, info, flags)?;

    Ok(())
}

fn convert<'a, I>(args: I) -> Result<Vec<String>>
where
    I: Iterator<Item = &'a CStr16>,
{
    args.skip(1)
        .map(|arg| {
            let buf = arg.to_u16_slice();
            let s = String::from_utf16(buf)?;

            Ok(s)
        })
        .collect()
}

/// REWRITE & DELETE
/// REWRITE & DELETE
/// REWRITE & DELETE
fn parse(args: &[String], flags: &mut Flags) -> Result<()> {
    for arg in args {
        let arg = arg.as_str();

        if let Some(val) = arg
            .strip_prefix("-p=")
            .or_else(|| arg.strip_prefix("--page="))
        {
            flags.page = match val {
                "main" | "MAIN" => Page::Main,
                "firm" | "FIRM" => Page::Firmware,
                "mem" | "MEM" => Page::Memory,
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
                flags.invalid_flag = true;
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
    fn print_err(&self, stdout: &mut ScopedProtocol<Output>) -> fmt::Result {
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

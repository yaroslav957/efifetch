#![no_std]
#![no_main]

extern crate alloc;

use crate::{
    error::Result,
    info::Info,
    output::{draw, page::Page, theme::Theme},
};

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::Write;
use tenu::lex::{ArgType, LookupTable, Parser, Token};
use uefi::{
    Error, Status,
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

const TABLE: LookupTable = LookupTable(&[
    ("help", ArgType::None, 'h'),
    ("version", ArgType::None, 'v'),
    ("page", ArgType::Required, 'p'),
    ("theme", ArgType::Required, 't'),
]);

const HELP: &str = concat!(
    "Usage: efifetch [OPTION]...\n",
    "A simple system information tool for EFI environments.\n",
    "\n",
    "Options:\n",
    "  -h, --help           display this help and exit\n",
    "  -v, --version        output version information and exit\n",
    "  -p, --page=PAGE      specify the information page to display\n",
    "                         (main, firm, mem)\n",
    "  -t, --theme=COLOR    set the output color theme\n",
    "                         (red, green)\n",
    "\n",
    "Examples:\n",
    "  efifetch -p=mem   Show only memory info\n",
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

    let args = params
        .args()
        .skip(1)
        .map(|arg| arg.to_string())
        .collect::<Vec<_>>();

    let info = Info::new()?;
    let mut page = Page::default();
    let mut theme = Theme::default();

    parse(&mut stdout, &args, &mut page, &mut theme)?;
    draw(&mut stdout, info, page, theme)?;

    Ok(())
}

fn parse(
    stdout: &mut ScopedProtocol<Output>,
    args: &[String],
    page: &mut Page,
    theme: &mut Theme,
) -> Result<()> {
    let args = args.iter().map(|arg| arg.as_str());
    let parser = Parser::new(args, TABLE);

    for token in parser {
        match token {
            Token::Option("help", _) => {
                writeln!(stdout, "{HELP}")?;
                return Err(Error::new(Status::ABORTED, ()).into());
            }
            Token::Option("version", _) => {
                writeln!(stdout, "Version: {VERSION} | {TIMESTAMP}")?;
                writeln!(stdout, "Built on: {RUSTC} Rust")?;
                return Err(Error::new(Status::ABORTED, ()).into());
            }
            Token::Option("page", Some(val)) => {
                *page = match val.to_lowercase().as_str() {
                    "main" => Page::Main,
                    "firm" => Page::Firmware,
                    "mem" => Page::Memory,
                    _ => {
                        writeln!(stdout, "Unexpected page value: '{val}'")?;
                        return Err(
                            Error::new(Status::INVALID_PARAMETER, ()).into()
                        );
                    }
                };
            }
            Token::Option("theme", Some(val)) => {
                *theme = match val.to_lowercase().as_str() {
                    "red" => Theme::RED,
                    "green" => Theme::GREEN,
                    _ => {
                        writeln!(stdout, "Unexpected theme value: '{val}'")?;
                        return Err(
                            Error::new(Status::INVALID_PARAMETER, ()).into()
                        );
                    }
                };
            }
            Token::Error(err) => {
                writeln!(stdout, "Internal parser error: '{err}'")?;
                return Err(Error::new(Status::ABORTED, ()).into());
            }
            Token::Value(val) => {
                writeln!(stdout, "Unexpected argument: '{val}'")?;
                return Err(Error::new(Status::ABORTED, ()).into());
            }
            _ => {}
        }
    }

    Ok(())
}

#![no_std]
#![no_main]

mod consts;
mod display;
mod info;

use crate::{
    consts::{KEY_A, KEY_E, KEY_M},
    display::{Display, page::Page},
    info::Info,
};
use core::time::Duration;
use uefi::{
    Error, Result, ResultExt, Status,
    boot::{
        ScopedProtocol, get_handle_for_protocol, open_protocol_exclusive, stall, wait_for_event,
    },
    entry, helpers,
    proto::console::text::{
        Input,
        Key::{Printable, Special},
        Output, ScanCode,
    },
};

pub type In = ScopedProtocol<Input>;
pub type Out = ScopedProtocol<Output>;

#[entry]
pub fn main() -> Status {
    helpers::init().unwrap();

    let out_handle = get_handle_for_protocol::<Output>().unwrap();
    let mut out = open_protocol_exclusive(out_handle).unwrap();

    let inp_handle = get_handle_for_protocol::<Input>().unwrap();
    let mut inp = open_protocol_exclusive(inp_handle).unwrap();

    event_handler(&mut inp, &mut out).unwrap();
    stall(Duration::from_secs(1));

    Status::SUCCESS
}

pub fn event_handler(inp: &mut In, out: &mut Out) -> Result<()> {
    let info = Info::new()?;
    let mut display = Display::new(out)?;

    display.draw_topbar(out);
    display.main_page(out);

    loop {
        let mut events = [inp
            .wait_for_key_event()
            .ok_or(Error::new(Status::UNSUPPORTED, ()))?];
        wait_for_event(&mut events).discard_errdata()?;

        if let Some(key) = inp.read_key()? {
            match key {
                Printable(KEY_M) => display.main_page(out),
                Printable(KEY_A) => display.about_page(out, &info),
                Printable(KEY_E) => break,

                Special(ScanCode::DOWN) => {
                    if display.page == Page::Main {
                        display.next_category(out)
                    }
                }

                Special(ScanCode::UP) => {
                    if display.page == Page::Main {
                        display.prev_category(out)
                    }
                }
                _ => (),
            }
        }
    }

    Ok(())
}

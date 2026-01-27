#![no_std]
#![no_main]

mod consts;
mod info;
mod tui;
mod utils;

use crate::{info::Info, tui::Canvas};
use core::time::Duration;
use uefi::{
    Error, Result, ResultExt, Status,
    boot::{
        ScopedProtocol, get_handle_for_protocol, open_protocol_exclusive, stall, wait_for_event,
    },
    entry, helpers,
    proto::console::text::{Input, Output},
};

pub type Inp = ScopedProtocol<Input>;
pub type Out = ScopedProtocol<Output>;

#[entry]
pub fn main() -> Status {
    helpers::init().unwrap();

    let inp_handle = get_handle_for_protocol::<Input>().unwrap();
    let out_handle = get_handle_for_protocol::<Output>().unwrap();

    // panic on inp
    let mut inp = open_protocol_exclusive::<Input>(out_handle).unwrap();
    let mut out = open_protocol_exclusive::<Output>(inp_handle).unwrap();

    let info = Info::new().unwrap();
    let mut canvas = tui::Canvas::new(inp, out).unwrap();

    event_handler(&mut canvas, info).unwrap();

    // Exit without boot-options jumpscare
    stall(Duration::from_secs(2));
    Status::SUCCESS
}

pub fn event_handler(canvas: &mut Canvas, _info: Info) -> Result<()> {
    loop {
        let mut events = [canvas
            .input()
            .wait_for_key_event()
            .ok_or(Error::new(Status::UNSUPPORTED, ()))?];
        wait_for_event(&mut events).discard_errdata()?;

        tui::on_draw(|canvas| {
            canvas.init_grid()?;
            uefi::Result::Ok(())
        })?;
    }
}

#![no_main]
#![no_std]

mod display;
mod event;
mod info;
mod utils;

use crate::event::event_handler;
use uefi::{
    Status,
    boot::{ScopedProtocol, get_handle_for_protocol, open_protocol_exclusive},
    entry, helpers,
    proto::console::text::{Input, Output},
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

    event_handler(&mut inp, &mut out).unwrap()
}

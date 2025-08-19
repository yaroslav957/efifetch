#![no_main]
#![no_std]

extern crate alloc;

mod event;
mod info;

use crate::event::event_loop;
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

    event_loop(&mut inp, &mut out).unwrap()
}

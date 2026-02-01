#![no_std]
#![no_main]

mod consts;
mod error;
mod info;
mod tui;
mod utils;

use crate::{
    error::Result,
    info::Info,
    tui::{Canvas, Theme},
};
use core::time::Duration;
use uefi::{
    Status,
    boot::{get_handle_for_protocol, open_protocol_exclusive, stall},
    entry, helpers,
    proto::console::text::{Input, Output},
};

#[entry]
pub fn main() -> Status {
    helpers::init().unwrap();

    let inp_handle = get_handle_for_protocol::<Input>().unwrap();
    let out_handle = get_handle_for_protocol::<Output>().unwrap();

    let inp = open_protocol_exclusive::<Input>(inp_handle).unwrap();
    let out = open_protocol_exclusive::<Output>(out_handle).unwrap();

    let _info = Info::new().unwrap();
    let theme = Theme::default();
    let canvas = tui::Canvas::new(inp, out, theme).unwrap();

    on_draw(canvas).unwrap();

    // Exit without boot-options jumpscare
    stall(Duration::from_secs(3));
    Status::SUCCESS
}

pub fn on_draw(mut canvas: Canvas) -> Result<()> {
    canvas.draw_topbar()?.draw_grid()?;
    Ok(())
}

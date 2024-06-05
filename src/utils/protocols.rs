use uefi::Handle;
use uefi::prelude::BootServices;
use uefi::proto::console::text::{Color, Output, OutputMode};
use uefi::proto::Protocol;
use uefi::table::boot::ScopedProtocol;

pub fn open_scoped<T: Protocol>(bt: &BootServices) -> ScopedProtocol<T> {
    let protocol_handle: Handle = bt.get_handle_for_protocol::<T>()
        .expect("Cant create handle");
    bt.open_protocol_exclusive::<T>(protocol_handle)
        .expect("Cant open protocol")
}

pub fn get_resolution(stdout: &mut Output) -> (usize, usize) {
    let output_mode: OutputMode = stdout.current_mode()
        .expect("Option").expect("Result");
    (output_mode.rows(), output_mode.columns())
}

pub fn change_text_color(stdout: &mut Output, color: Color) -> () {
    const BLACK: Color = Color::Black;
    stdout.set_color(color, BLACK)
        .expect("Cant change color");
}
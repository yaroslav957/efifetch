use uefi::prelude::BootServices;
use uefi::proto::console::text::{Color, Output};
use uefi::proto::Protocol;
use uefi::table::boot::ScopedProtocol;

#[inline]
pub fn open_scoped<T: Protocol>(bt: &BootServices) -> ScopedProtocol<T> {
    let protocol_handle = bt.get_handle_for_protocol::<T>()
        .expect("Cant create handle");
    bt.open_protocol_exclusive::<T>(protocol_handle)
        .expect("Cant open protocol")
}

#[inline]
pub fn get_resolution(stdout: &mut Output) -> (usize, usize) {
    let output_mode = stdout.current_mode()
        .ok().flatten().unwrap();
    (output_mode.rows(), output_mode.columns())
}

#[inline]
pub fn stdout_text_color(stdout: &mut Output, color: Color) {
    stdout.set_color(color, Color::Black)
        .expect("Cant change color");
}
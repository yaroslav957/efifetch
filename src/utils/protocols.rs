use uefi::prelude::BootServices;
use uefi::proto::console::text::{Color, Output};
use uefi::proto::Protocol;
use uefi::table::boot::ScopedProtocol;

#[inline]
pub(crate) fn open_scoped<T: Protocol>(bt: &BootServices) -> ScopedProtocol<T> {
    let protocol_handle = bt
        .get_handle_for_protocol::<T>()
        .expect("Cant create handle");
    bt.open_protocol_exclusive::<T>(protocol_handle)
        .expect("Cant open protocol")
}

#[inline]
pub(crate) fn get_resolution(stdout: &mut Output) -> (usize, usize) {
    let output_mode = stdout
        .current_mode()
        .ok()
        .flatten()
        .expect("Cant take output mode");
    (output_mode.rows(), output_mode.columns())
}

#[inline]
pub(crate) fn stdout_text_color(stdout: &mut Output, color: Color) {
    stdout
        .set_color(color, Color::Black)
        .expect("Cant change color");
}

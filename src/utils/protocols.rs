use uefi::{
    boot::ScopedProtocol,
    proto::{
        console::text::{Color, Output},
        Protocol,
    },
    Result,
};

#[inline]
pub fn open_scoped<T: Protocol>() -> Result<ScopedProtocol<T>> {
    let proto_handle = uefi::boot::get_handle_for_protocol::<T>()?;
    uefi::boot::open_protocol_exclusive(proto_handle)
}

#[inline]
pub fn get_resolution(stdout: &mut Output) -> Result<(usize, usize)> {
    let output_mode = stdout.current_mode()?.unwrap();
    Ok((output_mode.rows(), output_mode.columns()))
}

#[inline]
pub fn stdout_text_color(stdout: &mut Output, color: Color) -> Result<()> {
    stdout.set_color(color, Color::Black)
}

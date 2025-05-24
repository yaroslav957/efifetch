mod draw;
mod info;

use {
    info::Info,
    uefi::{
        Result, Status,
        boot::ScopedProtocol,
        proto::console::text::{Input, Output},
    },
};

pub type Stdin = ScopedProtocol<Input>;
pub type Stdout = ScopedProtocol<Output>;

pub fn main_eventloop() -> Result<Status> {
    uefi::helpers::init().unwrap();

    let info = Info::new()?;

    let output_handle = uefi::boot::get_handle_for_protocol::<Output>()?;
    let mut stdout = uefi::boot::open_protocol_exclusive(output_handle)?;

    let input_handle = uefi::boot::get_handle_for_protocol::<Input>()?;
    let mut stdin = uefi::boot::open_protocol_exclusive(input_handle)?;

    draw::draw(&mut stdin, &mut stdout, &info)?;

    Ok(Status::SUCCESS)
}

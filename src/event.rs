use crate::{In, Out, info::Info};
use uefi::{
    Error, Result, Status,
    boot::{stall, wait_for_event},
    proto::console::text::{Key::Special, ScanCode},
};

const STALL_TIME: usize = 700_000;
const MIN_CONSOLE_WIDTH: usize = 80;
const MIN_CONSOLE_HEIGHT: usize = 25;

pub fn event_loop(inp: &mut In, out: &mut Out) -> Result<Status> {
    let _ = Info::new();

    let [width, height] = resolution(out)?;
    if width < MIN_CONSOLE_WIDTH || height < MIN_CONSOLE_HEIGHT {
        return Err(Error::new(Status::UNSUPPORTED, ()));
    }

    out.clear()?;
    maximize(out)?;

    loop {
        let mut events = [inp.wait_for_key_event().unwrap()];
        wait_for_event(&mut events).unwrap();

        if let Some(key) = inp.read_key()? {
            match key {
                Special(ScanCode::FUNCTION_1) => (),
                Special(ScanCode::FUNCTION_2) => (),
                Special(ScanCode::DELETE) => break,
                _ => continue,
            }
        }
    }

    stall(STALL_TIME);
    Ok(Status::SUCCESS)
}

pub fn resolution(out: &Out) -> Result<[usize; 2]> {
    let mode = out.current_mode()?.unwrap();
    Ok([mode.rows(), mode.columns()])
}

#[cold]
fn maximize(out: &mut Out) -> Result<()> {
    let max_mode = out.modes().last().unwrap();
    out.set_mode(max_mode)
}

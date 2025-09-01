use crate::{
    In, Out,
    display::Display,
    info::Info,
    utils::{minimize, resolution},
};
use uefi::{
    Error, Result, Status,
    boot::{stall, wait_for_event},
    proto::console::text::{Key::Special, ScanCode},
};

const MIN_CONSOLE_WIDTH: usize = 80;
const MIN_CONSOLE_HEIGHT: usize = 25;

pub fn event_handler(inp: &mut In, out: &mut Out) -> Result<Status> {
    out.clear()?;
    minimize(out)?;

    let display = Display::new(out)?;

    let [width, height] = resolution(out)?;

    if width < MIN_CONSOLE_WIDTH || height < MIN_CONSOLE_HEIGHT {
        return Err(Error::new(Status::UNSUPPORTED, ()));
    }

    display.topbar(out)?;
    display.startscreen(out)?;
    let info = Info::new()?;
    display.bottombar(out)?;

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

    stall(700_000);
    Ok(Status::SUCCESS)
}

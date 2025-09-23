use crate::{
    In, Out,
    display::Display,
    info::Info,
    utils::{check_resolution, minimize, resolution},
};
use uefi::{
    Result, Status,
    boot::{stall, wait_for_event},
    proto::console::text::{Key::Special, ScanCode},
};

pub fn event_handler(inp: &mut In, out: &mut Out) -> Result<Status> {
    out.clear()?;
    minimize(out)?;

    let info = Info::new()?;
    let [width, height] = resolution(out)?;
    let display = Display::new(out)?;

    check_resolution(width, height)?.0;

    display.top_bar(out)?;
    display.start_page(out)?;

    loop {
        let mut events = [inp.wait_for_key_event().unwrap()];
        wait_for_event(&mut events).unwrap();

        if let Some(key) = inp.read_key()? {
            match key {
                Special(ScanCode::FUNCTION_1) => (),
                Special(ScanCode::FUNCTION_2) => (),
                Special(ScanCode::DELETE) => break,
                _ => display.memory_page(out, &info).unwrap(), //TODO
            }
        }
    }

    stall(700_000);
    Ok(Status::SUCCESS)
}

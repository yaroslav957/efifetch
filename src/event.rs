use crate::{
    In, Out,
    display::{Display, page::Page},
    info::Info,
    utils::{check_resolution, minimize, resolution},
};
use uefi::{
    Char16, Result, Status,
    boot::{stall, wait_for_event},
    proto::console::text::{
        Key::{Printable, Special},
        ScanCode,
    },
};

const KEY_M: Char16 = unsafe { Char16::from_u16_unchecked(0x006D) };
const KEY_A: Char16 = unsafe { Char16::from_u16_unchecked(0x0061) };
const KEY_E: Char16 = unsafe { Char16::from_u16_unchecked(0x0065) };

pub fn event_handler(inp: &mut In, out: &mut Out) -> Result<Status> {
    out.clear()?;
    minimize(out)?;

    let _ = Info::new()?;
    let [width, height] = resolution(out)?;
    let mut display = Display::new(out)?;

    check_resolution(width, height)?.0;

    display.topbar(out)?;
    display.main_page(out)?;

    loop {
        let mut events = [inp.wait_for_key_event().unwrap()];
        wait_for_event(&mut events).unwrap();

        if let Some(key) = inp.read_key()? {
            match key {
                Printable(KEY_M) => display.main_page(out)?,
                Printable(KEY_A) => display.about_page(out)?,
                Printable(KEY_E) => break,

                Special(ScanCode::DOWN) => {
                    if display.page() == Page::Main {
                        display.next_category(out)
                    }
                }

                Special(ScanCode::UP) => {
                    if display.page() == Page::Main {
                        display.prev_category(out)
                    }
                }
                _ => (),
            }
        }
    }

    stall(700_000);
    Ok(Status::SUCCESS)
}

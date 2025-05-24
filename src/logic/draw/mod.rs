pub mod cpu;
pub mod mem;
pub mod menu;

use {
    crate::logic::{Info, Stdin, Stdout},
    alloc::vec::Vec,
    uefi::{
        Result,
        proto::console::text::{Key, ScanCode},
    },
};

pub fn draw(stdin: &mut Stdin, stdout: &mut Stdout, info: &Info) -> Result<()> {
    stdout.clear()?;
    maximese_resolution(stdout)?;
    menu::draw(stdout, info.date)?;

    loop {
        if let Some(key) = stdin.read_key()? {
            match key {
                Key::Special(ScanCode::DELETE) => {
                    uefi::boot::stall(700_000);
                    break;
                }
                Key::Special(ScanCode::ESCAPE) => menu::draw(stdout, info.date).unwrap(),
                Key::Special(ScanCode::FUNCTION_1) => (), // PCI
                Key::Special(ScanCode::FUNCTION_2) => cpu::draw(&info.cpu_info),
                Key::Special(ScanCode::FUNCTION_3) => mem::draw(&info.mem_info),
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn _get_resolution(stdout: &mut Stdout) -> Result<(usize, usize)> {
    let mode = stdout.current_mode()?.unwrap();
    Ok((mode.rows(), mode.columns()))
}

pub fn maximese_resolution(stdout: &mut Stdout) -> Result<()> {
    let modes = stdout.modes().collect::<Vec<_>>();
    stdout.set_mode(*modes.last().unwrap())
}

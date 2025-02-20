use crate::{logic::draw::menu, utils};
use uefi::proto::console::text::Output;

mod draw;
mod info;

pub fn main_loop() -> ! {
    uefi::helpers::init().expect("Cant init helpers");
    let mut stdout = utils::protocols::open_scoped::<Output>()
        .expect("[efifetch]: error caused by opening stdout proto");

    let _ = menu::draw(&mut stdout).expect("[efifetch]: error while drawing the menu");

    loop {}
}

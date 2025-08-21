use crate::{Out, utils::resolution};
use core::fmt::Write;
use uefi::{Result, println, proto::console::text::Color};

const LOGO: &str = include_str!("./assets/uefi.logo");
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub trait Draw {
    fn draw(&self) -> Result<()>;
}

pub fn draw_top(out: &mut Out) -> Result<()> {
    out.set_color(Color::LightGray, Color::Blue)?;

    let [width, height] = resolution(out)?;
    println!("        zzz                              zzzzzzz\n");
    Ok(())
}

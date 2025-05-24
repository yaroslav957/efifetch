use {
    crate::logic::{Stdout, info::date::Date},
    core::fmt::Write,
    uefi::{Result, proto::console::text::Color},
};

const LOGO: &'static str = include_str!("../../assets/uefi.logo");
const _VERSION: &'static str = "Efifetch 0.1.9";

pub fn draw(stdout: &mut Stdout, _date: Date) -> Result<()> {
    stdout.set_color(Color::Red, Color::Black)?;
    let mut logo = LOGO.lines();

    for _ in 0..LOGO.lines().count() {
        stdout.write_str("                              ").unwrap();
        stdout.write_fmt(format_args!("{}", logo.next().expect("on {i} iter"))).unwrap();
        stdout.write_str("                              ").unwrap();
    }
    Ok(())
}

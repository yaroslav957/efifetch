use crate::{
    error::Result,
    info::{date::Date, firmware::Firmware, memory::Memory},
};

use alloc::string::String;
use core::fmt::{self, Write};

mod date;
mod firmware;
mod memory;

pub trait InfoItem {
    fn render(&self) -> impl Iterator<Item = (&str, &str)>;
}

trait FromArgs {
    fn build(args: fmt::Arguments) -> Result<String>;
}

impl FromArgs for String {
    fn build(args: fmt::Arguments) -> Result<Self> {
        let mut s = String::new();
        s.write_fmt(args)?;

        Ok(s)
    }
}

#[derive(Clone)]
pub struct Info {
    pub date: Date,
    pub firmware: Firmware,
    pub memory: Memory,
}

impl Info {
    pub fn new() -> Result<Self> {
        let date = Date::new()?;
        let firmware = Firmware::new()?;
        let memory = Memory::new()?;

        Ok(Self {
            date,
            firmware,
            memory,
        })
    }
}

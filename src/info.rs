mod date;
mod env;
mod firmware;
mod memory;

use crate::{
    error::Result,
    info::{date::Date, env::Env, firmware::Firmware, memory::Memory},
};
use core::fmt::{self, Write};
use heapless::String;

pub trait InfoItem {
    fn render(&self) -> impl Iterator<Item = (&str, &str)>;
}

trait FromArgs<const N: usize> {
    fn build(args: fmt::Arguments) -> Result<String<N>>;
}

impl<const N: usize> FromArgs<N> for String<N> {
    fn build(args: fmt::Arguments) -> Result<Self> {
        let mut s = String::new();
        s.write_fmt(args)?;
        Ok(s)
    }
}

#[derive(Clone)]
pub struct Info {
    pub date: Date,
    pub env: Env,
    pub firmware: Firmware,
    pub memory: Memory,
}

impl Info {
    pub fn new() -> Result<Self> {
        let date = Date::new()?;
        let env = Env::new()?;
        let firmware = Firmware::new()?;
        let memory = Memory::new()?;

        Ok(Self {
            date,
            env,
            firmware,
            memory,
        })
    }
}

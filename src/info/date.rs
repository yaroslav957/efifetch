use crate::{error::Result, info::InfoItem};
use core::fmt::Write;
use heapless::String;
use uefi::runtime::get_time;

#[derive(Clone)]
pub struct Date {
    pub date: String<16>,
    pub time: String<8>,
}

impl Date {
    pub fn new() -> Result<Self> {
        let time = get_time()?;

        let date = {
            let mut s = String::new();
            write!(&mut s, "{}/{}/{}", time.day(), time.month(), time.year())?;
            s
        };

        let time = {
            let mut s = String::new();
            write!(&mut s, "{}:{}", time.hour(), time.minute())?;
            s
        };

        Ok(Self { date, time })
    }
}

impl InfoItem for Date {
    fn render(&self) -> impl Iterator<Item = (&str, &str)> {
        [("Date:", self.date.as_str()), ("Time:", self.time.as_str())]
            .into_iter()
    }
}

use crate::{
    error::Result,
    info::{FromArgs, InfoItem},
};

use alloc::string::String;

use uefi::runtime::get_time;

#[derive(Clone)]
pub struct Date {
    pub date: String,
    pub time: String,
}

impl Date {
    pub fn new() -> Result<Self> {
        let time = get_time()?;

        let date = String::build(format_args!(
            "{:02}/{:02}/{}",
            time.day(),
            time.month(),
            time.year()
        ))?;
        let time = String::build(format_args!(
            "{:02}:{:02} (UTC)",
            time.hour(),
            time.minute()
        ))?;

        Ok(Self { date, time })
    }
}

impl InfoItem for Date {
    fn render(&self) -> impl Iterator<Item = (&str, &str)> {
        [("Date:", self.date.as_str()), ("Time:", self.time.as_str())]
            .into_iter()
    }
}

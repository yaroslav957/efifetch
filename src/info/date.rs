use crate::error::Result;
use uefi::runtime::get_time;

#[derive(Clone, Copy)]
pub struct Date {
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub fn new() -> Result<Self> {
        let time = get_time()?;

        let hour = time.hour();
        let minute = time.minute();

        let day = time.day();
        let month = time.month();
        let year = time.year();

        Ok(Self {
            hour,
            minute,
            day,
            month,
            year,
        })
    }

    pub fn time(&self) -> (u8, u8) {
        // hh:mm
        (self.hour, self.minute)
    }

    pub fn date(&self) -> (u8, u8, u16) {
        // dd:mm:yyyy
        (self.day, self.month, self.year)
    }
}

use crate::{error::Result, utils::U32Buffer};
use uefi::runtime::get_time;

#[derive(Clone, Copy)]
pub struct Date {
    day: U32Buffer,
    hour: U32Buffer,
    minute: U32Buffer,
    month: U32Buffer,
    year: U32Buffer,
}

impl Date {
    pub fn new() -> Result<Self> {
        let time = get_time()?;

        let hour = U32Buffer::new(time.hour() as u32);
        let minute = U32Buffer::new(time.minute() as u32);

        let day = U32Buffer::new(time.day() as u32);
        let month = U32Buffer::new(time.month() as u32);
        let year = U32Buffer::new(time.year() as u32);

        Ok(Self {
            hour,
            minute,
            day,
            month,
            year,
        })
    }

    pub fn time(&self) -> [U32Buffer; 2] {
        // hh:mm
        [self.hour, self.minute]
    }

    pub fn date(&self) -> [U32Buffer; 3] {
        // dd:mm:yyyy
        [self.day, self.month, self.year]
    }
}

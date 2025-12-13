use crate::info::U32Buffer;
use uefi::Result;

pub struct Date {
    pub hour: U32Buffer,
    pub minute: U32Buffer,
    pub day: U32Buffer,
    pub month: U32Buffer,
    pub year: U32Buffer,
}

impl Date {
    pub fn new() -> Result<Self> {
        let time = uefi::runtime::get_time()?;

        let day = U32Buffer::new(time.day() as u32);
        let month = U32Buffer::new(time.month() as u32);
        let year = U32Buffer::new(time.year() as u32);

        let hour = U32Buffer::new(time.hour() as u32);
        let minute = U32Buffer::new(time.minute() as u32);

        Ok(Self {
            hour,
            minute,
            day,
            month,
            year,
        })
    }
}

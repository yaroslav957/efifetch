use uefi::Result;

pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub fn get() -> Result<Self> {
        let time = uefi::runtime::get_time()?;
        Ok(Self {
            day: time.day(),
            month: time.month(),
            year: time.year(),
        })
    }
}

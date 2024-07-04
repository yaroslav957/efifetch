use uefi::prelude::RuntimeServices;
use uefi::table::runtime::Time;

pub(crate) struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
}

impl From<Time> for Date {
    fn from(time: Time) -> Self {
        Self {
            day: time.day(),
            month: time.month(),
            year: time.year(),
        }
    }
}

impl Date {
    pub(crate) fn get(runtime_services: &RuntimeServices) -> Self {
        runtime_services.get_time()
            .unwrap().into()
    }
}
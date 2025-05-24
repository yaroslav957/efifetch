pub mod cpu;
pub mod date;
pub mod mem;

use {cpu::CpuInfo, date::Date, mem::MemInfo, uefi::Result};

pub struct Info {
    pub date: Date,
    pub cpu_info: CpuInfo,
    pub mem_info: MemInfo,
}

impl Info {
    pub fn new() -> Result<Self> {
        Ok(Self {
            date: Date::get()?,
            cpu_info: CpuInfo::get(),
            mem_info: MemInfo::get()?,
        })
    }
}

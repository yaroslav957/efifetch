use crate::{
    error::Result,
    info::{Info, InfoItem},
};
use heapless::{CapacityError, Vec};

#[derive(Clone, Copy, Default)]
pub enum Page {
    #[default]
    Main,
    Env,
    Firmware,
    Memory,
}

impl Page {
    pub fn add<'r, const N: usize>(
        &self,
        rows: &mut Vec<(&'r str, &'r str), N>,
        info: &'r Info,
    ) -> Result<()> {
        match self {
            Self::Main => {
                Self::filter(rows, &info.date, |_| true)?;

                let allowed = ["Vendor:", "UEFI revision:"];
                Self::filter(rows, &info.firmware, |(label, _)| {
                    allowed.contains(label)
                })?;

                let allowed = ["Memory:"];
                Self::filter(rows, &info.memory, |(label, _)| {
                    allowed.contains(label)
                })?;
            }
            Self::Env => Self::filter(rows, &info.env, |_| true)?,
            Self::Firmware => Self::filter(rows, &info.firmware, |_| true)?,
            Self::Memory => Self::filter(rows, &info.memory, |_| true)?,
        }

        Ok(())
    }

    fn filter<'r, T, const N: usize, F>(
        rows: &mut Vec<(&'r str, &'r str), N>,
        item: &'r T,
        filter: F,
    ) -> Result<()>
    where
        T: InfoItem,
        F: FnMut(&(&'r str, &'r str)) -> bool,
    {
        for row in item.render().filter(filter) {
            rows.push(row).map_err(|_| CapacityError::default())?;
        }

        Ok(())
    }
}

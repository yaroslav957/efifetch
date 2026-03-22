use crate::{
    error::Result,
    info::{Info, InfoItem},
};

use alloc::vec::Vec;

#[derive(Clone, Copy, Default)]
pub enum Page {
    #[default]
    Main,
    Firmware,
    Memory,
}

impl Page {
    pub fn add<'r>(
        &self,
        rows: &mut Vec<(&'r str, &'r str)>,
        info: &'r Info,
    ) -> Result<()> {
        match self {
            Self::Main => {
                let allowed = ["Secure Boot:", "Language:"];
                Self::filter(rows, &info.firmware, |(label, _)| {
                    !allowed.contains(label)
                })?;

                let allowed = ["Memory:"];
                Self::filter(rows, &info.memory, |(label, _)| {
                    allowed.contains(label)
                })?;
            }
            Self::Firmware => Self::filter(rows, &info.firmware, |_| true)?,
            Self::Memory => Self::filter(rows, &info.memory, |_| true)?,
        }

        Ok(())
    }

    fn filter<'r, T, F>(
        rows: &mut Vec<(&'r str, &'r str)>,
        item: &'r T,
        filter: F,
    ) -> Result<()>
    where
        T: InfoItem,
        F: FnMut(&(&'r str, &'r str)) -> bool,
    {
        for row in item.render().filter(filter) {
            rows.push(row);
        }

        Ok(())
    }
}

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
}

impl Page {
    pub fn add<'r, const N: usize>(
        &self,
        rows: &mut Vec<(&'r str, &'r str), N>,
        info: &'r Info,
    ) -> Result<()> {
        match self {
            Self::Main => {
                self.add_item(rows, &info.date)?;
                self.add_item(rows, &info.firmware)?;
            }
            Self::Env => {
                self.add_item(rows, &info.env)?;
            }
        }

        Ok(())
    }

    fn add_item<'r, T, const N: usize>(
        &self,
        rows: &mut Vec<(&'r str, &'r str), N>,
        item: &'r T,
    ) -> Result<()>
    where
        T: InfoItem,
    {
        for row in item.render() {
            rows.push(row).map_err(|_| CapacityError::default())?;
        }

        Ok(())
    }
}

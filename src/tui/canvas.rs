use crate::{Inp, Out};
use uefi::{Error, Result, Status, proto::console::text::OutputMode};

pub struct Canvas {
    inp: Inp,
    out: Out,
    mode: OutputMode,
}

impl Canvas {
    const TL: char = '┌';
    const TT: char = '┬';
    const TR: char = '┐';
    const HL: char = '─';
    const VL: char = '│';
    const BL: char = '└';
    const BT: char = '┴';
    const BR: char = '┘';

    pub fn new(inp: Inp, mut out: Out) -> Result<Self> {
        // The minimum vendor-supported resolution is 80x31
        let mode = out
            .modes()
            .min()
            .ok_or(Error::new(Status::UNSUPPORTED, ()))?;

        Ok(Self { inp, out, mode })
    }

    pub fn input(&self) -> &Inp {
        &self.inp
    }

    pub fn init_grid(&mut self) -> Result<&Self> {
        &self.out.clear()?;
        Ok(self)
    }
}

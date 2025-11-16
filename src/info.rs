mod firmware;
mod memory;

use crate::info::{firmware::Firmware, memory::Memory};
use uefi::Result;

pub struct Info {
    pub firmware: Firmware,
    pub memory: Memory,
}

impl Info {
    pub fn new() -> Result<Self> {
        let firmware = Firmware::new();
        let memory = Memory::new()?;

        Ok(Self { firmware, memory })
    }
}

pub struct U32Buffer {
    pub buf: [u8; 10],
    pub len: usize,
}

impl U32Buffer {
    pub fn new(num: u32) -> Self {
        let mut buf = [0u8; 10];
        let len = U32Buffer::format(num, &mut buf);
        Self { buf, len }
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.buf[..self.len]).unwrap_or_default()
    }

    fn format(mut num: u32, buf: &mut [u8; 10]) -> usize {
        let mut pos = 10;
        let mut len = 10;

        if num == 0 {
            buf[0] = b'0';
            return 1;
        }

        while num > 0 {
            pos -= 1;
            buf[pos] = b'0' + (num % 10) as u8;
            num /= 10;
        }

        len = len - pos;

        if pos > 0 {
            for i in 0..len {
                buf[i] = buf[pos + i];
            }
        }

        len
    }
}

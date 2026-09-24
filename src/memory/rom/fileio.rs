//! # fileio
//! Handles conversion from raw bites into bitfields

use std::{fs::File, io::Read, path::Path};

use anyhow::Context;

use crate::{helpers::u32_to_usize, memory::bus::BusWidth};

pub(crate) struct Rom {
    memory: Vec<u8>,
}


const GAMEPAKSIZE: usize = 0x0A00_0000 - 0x0800_0000;

impl Rom {
    pub(crate) fn initialize() -> Self {
        Self {
            memory: Vec::with_capacity(GAMEPAKSIZE),
        }
    }
    pub(crate) fn load_rom(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let mut file =
            File::open(path).with_context(|| format!("Error opening file: {}", path.display()))?;
        self.memory.clear();
        #[allow(
            clippy::verbose_file_reads,
            reason = "No reason to reallocate the buffer"
        )]
        file.read_to_end(&mut self.memory)
            .with_context(|| format!("Error reading ROM from path: {}", path.display()))?;
        Ok(())
    }
    pub(crate) fn read(&self, addr: u32, width: BusWidth) -> u32 {
        let addr = u32_to_usize(addr);
        let width = width.vec_width();
        let mut range = match self.memory.get(addr..=addr.strict_add(width)) {
            Some(val) => val,
            None => &[0],
        }.iter();
        let mut array: [u8; 4] = [0,0,0,0];
        for i in 0..array.len() {
            match array.get_mut(i) {
                Some(val) => if let Some(mem) = range.next() {
                    *val = *mem;
                },
                None => unreachable!(),
            }
        }
        u32::from_le_bytes(array)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn errors_on_non_existent_file() {
        let path = Path::new("arbitrary/path");
        let mut rom = Rom::initialize();

        let result = rom.load_rom(path);

        assert!(result.is_err());
    }
    #[test]
    fn opens_file() {
        let path = Path::new("./test-data/suite.gba");
        let mut rom = Rom::initialize();

        rom.load_rom(path).unwrap();

        assert_eq!(rom.read(0, BusWidth::B8), 0x2E);
    }
}

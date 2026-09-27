//! # fileio
//! Handles conversion from raw bites into bitfields

use std::{fs::File, io::Read, path::Path};

use anyhow::{Context, anyhow};

use crate::gba::{helpers::u32_to_usize, memory::bus::BusWidth};

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

    #[cfg_attr(not(test), expect(dead_code, reason = "Testing purposes"))]
    pub(crate) fn load_raw(&mut self, iter: impl Iterator<Item = u8>) {
        self.memory.clear();
        self.memory.extend(iter);
    }

    //TODO: Handle misaligned reads and determine a suitable open-bus descision.
    pub(crate) fn load_rom(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let mut file =
            File::open(path).with_context(|| format!("Error opening file: {}", path.display()))?;
        let file_size = usize::try_from(
            file.metadata()
                .with_context(|| format!("Error opening File Metadata for: {}", path.display()))?
                .len(),
        )?;

        if file_size > GAMEPAKSIZE {
            return Err(anyhow!("File too large to be a GBA ROM"));
        }

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
        let width = width.bytes();
        eprintln!("width: {width}");
        for test_val in self.memory.iter().skip(addr).take(width) {
            eprintln!("test_val: {test_val}");
        }
        let range = self.memory.iter().skip(addr).take(width);
        let mut array: [u8; 4] = [0, 0, 0, 0];
        for (dest, source) in array.iter_mut().zip(range) {
            eprintln!("source: {source}");
            *dest = *source;
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

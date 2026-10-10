//! # fileio
//! Handles conversion from raw bites into bitfields

use std::{fs::File, io::Read, path::Path};

use anyhow::{Context, anyhow};

use crate::gba::memory::{BusWidth, helpers::read_le_bytes};

pub(crate) struct Rom {
    memory: Vec<u8>,
}

const GAMEPAK_SIZE: usize = 0x0A00_0000 - 0x0800_0000;

impl Rom {
    pub(crate) fn new() -> Self {
        Self {
            memory: Vec::with_capacity(GAMEPAK_SIZE),
        }
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "Testing purposes"))]
    pub(crate) fn load_raw(&mut self, iter: impl Iterator<Item = u8>) {
        self.memory.clear();
        self.memory.extend(iter);
    }

    //TODO: Handle misaligned reads and determine a suitable open-bus descision.
    pub(crate) fn load(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        let mut file =
            File::open(path).with_context(|| format!("Error opening file: {}", path.display()))?;
        let file_size = usize::try_from(
            file.metadata()
                .with_context(|| format!("Error opening File Metadata for: {}", path.display()))?
                .len(),
        )?;

        if file_size > GAMEPAK_SIZE {
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
        read_le_bytes(&self.memory, addr, width)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn errors_on_non_existent_file() {
        let path = Path::new("arbitrary/path");
        let mut rom = Rom::new();

        let result = rom.load(path);

        assert!(result.is_err());
    }
    #[test]
    fn opens_file() {
        let path = Path::new("./test-data/suite.gba");
        let mut rom = Rom::new();

        rom.load(path).unwrap();

        assert_eq!(rom.read(0, BusWidth::B8), 0x2E);
    }
}

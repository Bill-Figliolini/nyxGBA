//! # fileio
//! Handles conversion from raw bites into bitfields

use std::{fs::File, io::Read, path::Path};

use anyhow::Context;

use crate::helpers::u32_to_usize;

pub(crate) struct Rom {
    memory: Vec<u8>,
}

const GAMEPAKSIZE: usize = 0x0E00_FFFF - 0x09FF_FFFF + 8;

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
        file.read(&mut self.memory)
            .with_context(|| format!("Error reading ROM from path: {}", path.display()))?;
        Ok(())
    }
    pub(crate) fn read8(&self, addr: u32) -> u32 {
        let addr = u32_to_usize(addr);
        match self.memory.get(addr) {
            Some(val) => u32::from(*val),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn errors_on_non_existent_file() {
        let dir = tempdir::TempDir::new("gba-test").unwrap();
        let path = dir.path().join("nonexistent");
        let mut rom = Rom::initialize();

        let result = rom.load_rom(path);

        assert!(result.is_err());
    }
}

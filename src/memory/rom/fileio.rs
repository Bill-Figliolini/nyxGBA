//! # fileio
//! Handles conversion from raw bites into bitfields

use std::path::Path;

use anyhow::Context;

use crate::helpers::u32_to_usize;

pub(crate) struct Rom {
    memory: Vec<u8>,
}

impl Rom {
    pub(crate) fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let memory: Vec<u8> = std::fs::read(path)
            .with_context(|| format!("Error reading ROM from path: {}", path.display()))?;
        Ok(Self { memory })
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

        let result = Rom::new(path);

        assert!(result.is_err());
    }
}

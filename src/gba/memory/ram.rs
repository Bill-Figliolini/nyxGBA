use crate::gba::memory::{
    BusWidth,
    helpers::{read_le_bytes, u32_to_usize},
};
pub(in crate::gba::memory) struct Ram {
    memory: Vec<u8>,
}

pub(in crate::gba::memory) const ONCHIPRAMSIZE: u32 = 0x8000;
pub(in crate::gba::memory) const ONBOARDRAMSIZE: u32 = 0x4_0000;

impl Ram {
    pub(in crate::gba::memory) fn initialize_on_chip() -> Self {
        Ram::initialize(u32_to_usize(ONCHIPRAMSIZE))
    }
    pub(in crate::gba::memory) fn initialize_on_board() -> Self {
        Ram::initialize(u32_to_usize(ONBOARDRAMSIZE))
    }
    fn initialize(size: usize) -> Self {
        Self {
            memory: vec![0; size],
        }
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "Testing purposes"))]
    pub(crate) fn load_raw(&mut self, iter: impl Iterator<Item = u8>) {
        for (dest, src) in self.memory.iter_mut().zip(iter) {
            *dest = src;
        }
    }
    pub(in crate::gba::memory) fn read(&self, address: u32, width: BusWidth) -> u32 {
        read_le_bytes(&self.memory, address, width)
    }
}

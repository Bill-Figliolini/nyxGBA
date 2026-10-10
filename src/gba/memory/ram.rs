use crate::gba::memory::{
    BusWidth,
    helpers::{read_le_bytes, u32_to_usize, write_le_bytes},
};
pub(in crate::gba::memory) struct Ram {
    memory: Vec<u8>,
}

pub(in crate::gba::memory) const ON_CHIP_RAM_SIZE: u32 = 0x8000;
pub(in crate::gba::memory) const ON_CHIP_RAM_MASK: u32 = ON_CHIP_RAM_SIZE - 1;
pub(in crate::gba::memory) const ON_BOARD_RAM_SIZE: u32 = 0x4_0000;
pub(in crate::gba::memory) const ON_BOARD_RAM_MASK: u32 = ON_BOARD_RAM_SIZE - 1;

impl Ram {
    pub(in crate::gba::memory) fn new_on_chip() -> Self {
        Ram::with_size(u32_to_usize(ON_CHIP_RAM_SIZE))
    }
    pub(in crate::gba::memory) fn new_on_board() -> Self {
        Ram::with_size(u32_to_usize(ON_BOARD_RAM_SIZE))
    }
    fn with_size(size: usize) -> Self {
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
    pub(in crate::gba::memory) fn write(&mut self, address: u32, width: BusWidth, value: u32) {
        write_le_bytes(&mut self.memory, address, width, value);
    }
}

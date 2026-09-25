#![expect(dead_code, reason = "Work in progress module")]

use super::rom::Rom;

struct MemoryBus {
    rom: Rom,
    counter: u32,
}
#[derive(Debug, Clone, Copy)]
struct Address(pub u32);

#[derive(Debug, Clone, Copy)]
pub(crate) enum BusWidth {
    B8,
    B16,
    B32,
}
impl BusWidth {
    pub(crate) fn vec_width(self) -> usize {
        match self {
            BusWidth::B8 => 0,
            BusWidth::B16 => 1,
            BusWidth::B32 => 3,
        }
    }
}

impl MemoryBus {
    fn read(&mut self, address: Address) -> u32 {
        self.counter = 0;
        match address.0 {
            //General
            0x0000_0000..=0x0000_3FFF => bios_read(),
            0x0200_0000..=0x0203_FFFF => board_memory_read(),
            0x0300_0000..=0x0300_7FFF => chip_memory_read(),
            0x0400_0000..=0x0400_03FE => io_memory_read(),

            //Display
            0x0500_0000..=0x0500_03FF => palette_memory_read(),
            0x0600_0000..=0x0601_7FFF => vram_read(),
            0x0700_0000..=0x0700_03FF => oam_read(),

            //External
            0x0800_0000..=0x0DFF_FFFF => {
                //Calculate waitestate from even thirds of addr
                let wait_state = match address.0 {
                    0x0800_0000..=0x09FF_FFFF => 0,
                    0x0A00_0000..=0x0BFF_FFFF => 1,
                    0x0C00_0000..=0x0DFF_FFFF => 2,
                    _ => unreachable!(),
                };
                gamepak_read(wait_state)
            }
            0x0E00_0000..=0x0E00_FFFF => gamepak_sram_read(),
            _ => unused_read(),
        }
    }
    fn write(&mut self, _address: Address, _value: u32) {
        self.counter = 0;
    }
}

fn bios_read() -> u32 {
    0
}
fn board_memory_read() -> u32 {
    0
}
fn chip_memory_read() -> u32 {
    0
}
fn io_memory_read() -> u32 {
    0
}
fn palette_memory_read() -> u32 {
    0
}
fn vram_read() -> u32 {
    0
}
fn oam_read() -> u32 {
    0
}
fn gamepak_read(_waitstate: u32) -> u32 {
    0
}
fn gamepak_sram_read() -> u32 {
    0
}
fn unused_read() -> u32 {
    0
}

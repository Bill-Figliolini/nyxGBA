#![expect(dead_code, reason = "Work in progress module")]

use std::path::Path;

use super::rom::Rom;

pub(in crate::gba) struct MemoryBus {
    pub rom: Rom,
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
    pub(crate) fn bytes(self) -> usize {
        match self {
            BusWidth::B8 => 1,
            BusWidth::B16 => 2,
            BusWidth::B32 => 4,
        }
    }
}

impl MemoryBus {
    pub(in crate::gba) fn startup() -> Self {
        Self {
            rom: Rom::initialize(),
            counter: 0,
        }
    }
    pub(in crate::gba) fn load_rom(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.rom.load_rom(path)
    }
    fn read(&mut self, address: Address, width: BusWidth) -> u32 {
        self.counter = 0;
        match address.0 {
            //General
            0x0000_0000..=0x0000_3FFF => MemoryBus::bios_read(),
            0x0200_0000..=0x0203_FFFF => MemoryBus::board_memory_read(),
            0x0300_0000..=0x0300_7FFF => MemoryBus::chip_memory_read(),
            0x0400_0000..=0x0400_03FE => MemoryBus::io_memory_read(),

            //Display
            0x0500_0000..=0x0500_03FF => MemoryBus::palette_memory_read(),
            0x0600_0000..=0x0601_7FFF => MemoryBus::vram_read(),
            0x0700_0000..=0x0700_03FF => MemoryBus::oam_read(),

            //External
            0x0800_0000..=0x0DFF_FFFF => {
                //Calculate waitstat from even thirds of addr
                let wait_state = match address.0 {
                    0x0800_0000..=0x09FF_FFFF => 0,
                    0x0A00_0000..=0x0BFF_FFFF => 1,
                    0x0C00_0000..=0x0DFF_FFFF => 2,
                    _ => unreachable!(),
                };
                self.gamepak_read(address, width, wait_state)
            }
            0x0E00_0000..=0x0E00_FFFF => MemoryBus::gamepak_sram_read(),
            _ => MemoryBus::unused_read(),
        }
    }
    fn write(&mut self, _address: Address, _value: u32) {
        self.counter = 0;
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
    fn gamepak_read(&mut self, address: Address, width: BusWidth, _waitstate: u32) -> u32 {
        let offset_address = address.0 & 0x01FF_FFFF;
        eprintln!("{offset_address}");
        self.rom.read(offset_address, width)
    }
    fn gamepak_sram_read() -> u32 {
        0
    }
    fn unused_read() -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod gamepak {
        use super::*;

        #[test]
        fn gamepak_read_waitstates_read_same_memory() {
            let mut bus = MemoryBus::startup();
            let vals: Vec<u8> = vec![0xAE];
            let output_val: u32 = 0xAE;
            bus.rom.load_raw(vals.iter().copied());

            assert_eq!(
                bus.gamepak_read(Address(0x0800_0000), BusWidth::B8, 0),
                output_val,
                "Waitstate 0 failed"
            );
            assert_eq!(
                bus.gamepak_read(Address(0x0A00_0000), BusWidth::B8, 1),
                output_val,
                "Waitstate 1 failed"
            );
            assert_eq!(
                bus.gamepak_read(Address(0x0C00_0000), BusWidth::B8, 2),
                output_val,
                "Waitstate 2 failed"
            );
        }

        #[test]
        fn gamepak_read_16b() {
            let mut bus = MemoryBus::startup();
            let vals: Vec<u8> = vec![0x34, 0x12];
            let output_val: u32 = 0x0000_1234;
            bus.rom.load_raw(vals.iter().copied());

            assert_eq!(
                bus.gamepak_read(Address(0x0800_0000), BusWidth::B16, 0),
                output_val,
                "Waitstate 0 failed"
            );
            assert_eq!(
                bus.gamepak_read(Address(0x0A00_0000), BusWidth::B16, 1),
                output_val,
                "Waitstate 1 failed"
            );
            assert_eq!(
                bus.gamepak_read(Address(0x0C00_0000), BusWidth::B16, 2),
                output_val,
                "Waitstate 2 failed"
            );
        }
        #[test]
        fn gamepak_read_32b() {
            let mut bus = MemoryBus::startup();
            let vals: Vec<u8> = vec![0x78, 0x56, 0x34, 0x12];
            let output_val: u32 = 0x1234_5678;
            bus.rom.load_raw(vals.iter().copied());

            assert_eq!(
                bus.gamepak_read(Address(0x0800_0000), BusWidth::B32, 0),
                output_val,
                "Waitstate 0 failed"
            );
            assert_eq!(
                bus.gamepak_read(Address(0x0A00_0000), BusWidth::B32, 1),
                output_val,
                "Waitstate 1 failed"
            );
            assert_eq!(
                bus.gamepak_read(Address(0x0C00_0000), BusWidth::B32, 2),
                output_val,
                "Waitstate 2 failed"
            );
        }
    }
}

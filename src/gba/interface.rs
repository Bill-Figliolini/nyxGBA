use std::{path::Path, process::abort};

use crate::gba::{
    bitmanip::Bitfield,
    cpu::{Cpu, startup},
    instructions,
    memory::{MemoryBus, bus::BusWidth},
};

pub(crate) struct Gba {
    cpu: Cpu,
    memory: MemoryBus,
}

impl Gba {
    pub(crate) fn startup() -> Self {
        Self {
            cpu: startup(),
            memory: MemoryBus::startup(),
        }
    }
    pub(crate) fn run(&mut self) {
        let instruction = instructions::parse(Bitfield::new(0));
        self.cpu.step(instruction);
        let path = Path::new("./test-data/suite.gba");
        let Ok(()) = self.memory.load_rom(path) else {
            abort()
        };
        let result = self.memory.rom.read(0, BusWidth::B8);
        println!("{result}");
    }
}

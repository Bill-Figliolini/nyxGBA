use std::{path::Path, process::abort};

use crate::gba::{bitmanip::Bitfield, cpu::{Cpu, startup}, instructions, memory::{bus::BusWidth, rom::Rom}};

pub(crate) struct Gba {
    cpu: Cpu
}

impl Gba {
    pub(crate) fn startup() -> Self {
        Self { cpu: startup() }
    }
    pub(crate) fn run(&mut self) {
        let instruction = instructions::parse(Bitfield::new(0));
        self.cpu.step(instruction);
        let path = Path::new("./test-data/suite.gba");
        let mut rom = Rom::initialize();
        let Ok(()) = rom.load_rom(path) else { abort() };
        let result = rom.read(0, BusWidth::B8);
        println!("{result}");
    }
}

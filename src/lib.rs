use std::{path::Path, process::abort};

use crate::{ bitmanip::Bitfield, cpu::Cpu, memory::rom::Rom};

mod bitmanip;
mod cpu;
mod helpers;
mod instructions;
mod memory;

pub fn nyx_main() {
    let mut cpu = Cpu::new();
    let instruction = memory::rom::parse(Bitfield::new(0));
    cpu.step(instruction);
    let path = Path::new("./test-data/suite.gba");
    let Ok(rom) = Rom::new(path) else { abort() };
    let result = rom.read8(0);
    println!("{result}");
}

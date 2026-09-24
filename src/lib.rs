use std::{path::Path, process::abort};

use crate::{bitmanip::Bitfield, memory::{bus::BusWidth, rom::Rom}};

mod bitmanip;
mod clock;
mod cpu;
mod helpers;
mod instructions;
mod memory;

#[cfg(target_pointer_width = "16")]
compile_error!("Only 32-bit and 64-bit architectures supported");
pub fn nyx_main() {
    let mut cpu = cpu::startup();
    let instruction = instructions::parse(Bitfield::new(0));
    cpu.step(instruction);
    let path = Path::new("./test-data/suite.gba");
    let mut rom = Rom::initialize();
    let Ok(()) = rom.load_rom(path) else { abort() };
    let result = rom.read(0, BusWidth::B8);
    println!("{result}");
}

use std::{path::Path, process::abort};

use crate::{bitmanip::Bitfield, memory::rom::Rom};

mod bitmanip;
mod clock;
mod cpu;
mod helpers;
mod instructions;
mod memory;

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Only 32-bit and 64-bit architectures supported");
pub fn nyx_main() {
    let mut cpu = cpu::startup();
    let instruction = memory::rom::parse(Bitfield::new(0));
    cpu.step(instruction);
    let path = Path::new("./test-data/suite.gba");
    let Ok(rom) = Rom::new(path) else { abort() };
    let result = rom.read8(0);
    println!("{result}");
}

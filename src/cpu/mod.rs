pub(crate) use registers::Register;

use crate::cpu::cpu_impl::Cpu;
mod arm_exec;
mod flags;
mod registers;
mod cpu_impl;

pub(crate) fn startup() -> Cpu {
    Cpu::new()
}

pub(crate) use registers::Register;

use crate::cpu::cpu_impl::Cpu;
mod arm_exec;
mod cpu_impl;
mod flags;
mod registers;

pub(crate) fn startup() -> Cpu {
    Cpu::new()
}

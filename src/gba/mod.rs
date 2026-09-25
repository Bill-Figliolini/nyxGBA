pub(in crate::gba) mod cpu;
pub(in crate::gba) mod instructions;
pub(crate) mod interface;
pub(in crate::gba) mod memory;
pub(in crate::gba) mod bitmanip;
pub(in crate::gba) mod clock;
pub(in crate::gba) mod helpers;

pub(crate) use interface::Gba;

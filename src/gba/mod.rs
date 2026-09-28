pub(in crate::gba) mod bitmanip;
pub(in crate::gba) mod clock;
pub(in crate::gba) mod cpu;
pub(crate) mod gba_impl;
pub(in crate::gba) mod instructions;
pub(in crate::gba) mod memory;

pub(crate) use gba_impl::Gba;

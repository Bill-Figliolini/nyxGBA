pub(crate) mod address;
pub(crate) mod bus;
pub(in crate::gba::memory) mod helpers;
pub(in crate::gba::memory) mod ram;
pub(crate) mod rom;
#[cfg(test)]
pub(crate) mod test_bus;

#[cfg(test)]
pub(in crate::gba) use test_bus::TestBus;
pub(in crate::gba) use {
    address::Address,
    bus::{Bus, BusWidth, MemoryBus},
};

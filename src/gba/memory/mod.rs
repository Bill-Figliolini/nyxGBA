pub(crate) mod bus;
pub(in crate::gba::memory) mod helpers;
pub(in crate::gba::memory) mod ram;
pub(crate) mod rom;
pub(crate) mod test_bus;
#[cfg_attr(not(test), expect(unused_imports, reason = "Testing purposes"))]
pub(in crate::gba) use {
    bus::{Bus, BusWidth, MemoryBus},
    test_bus::TestBus,
};

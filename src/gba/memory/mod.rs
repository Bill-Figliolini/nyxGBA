pub(crate) mod bus;
pub(crate) mod rom;
#[expect(
    unused_imports,
    reason = "Bus Width needed for Parsing, Memory Bus is interface to Memory"
)]
pub(in crate::gba) use bus::{BusWidth, MemoryBus};

use crate::gba::{bitmanip::Bitfield, memory::Bus};

pub(crate) struct TestBus {
    values: Box<dyn Iterator<Item = Bitfield>>,
}

impl TestBus {
    #[cfg_attr(not(test), expect(dead_code, reason = "Testing purposes"))]
    pub(crate) fn new(outputs: impl Iterator<Item = Bitfield> + 'static) -> Self {
        Self {
            values: Box::new(outputs),
        }
    }
}

impl Bus for TestBus {
    fn read(&mut self, _address: super::bus::Address, _width: super::BusWidth) -> Bitfield {
        self.values.next().unwrap_or_else(|| Bitfield::new(0))
    }

    fn write(&mut self, _address: super::bus::Address, _value: u32) {}
}

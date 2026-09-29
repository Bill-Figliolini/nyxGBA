use crate::gba::{
    bitmanip::Bitfield,
    memory::{Bus, BusWidth, bus::Address},
};

pub(crate) struct TestBus {
    read_inputs: Vec<(Address, BusWidth, Bitfield)>,
    write_inputs: Vec<(Address, BusWidth, Bitfield)>,
}

impl TestBus {
    pub(crate) fn new(
        read_inputs: Vec<(Address, BusWidth, Bitfield)>,
        write_inputs: Vec<(Address, BusWidth, Bitfield)>,
    ) -> Self {
        let read_inputs = read_inputs.into_iter().rev().collect();
        let write_inputs = write_inputs.into_iter().rev().collect();
        Self {
            read_inputs,
            write_inputs,
        }
    }
}

impl Bus for TestBus {
    fn read(&mut self, address: Address, width: BusWidth) -> Bitfield {
        if let Some((expected_address, expected_width, output)) = self.read_inputs.pop() {
            if expected_address == address && expected_width == width {
                output
            } else {
                panic!(
                    "Incorrect inputs: {:#x}, {}. Expected: {:#x}, {}",
                    address.0,
                    width.bytes(),
                    expected_address.0,
                    expected_width.bytes()
                )
            }
        } else {
            panic!("More calls to read than Expected!");
        }
    }

    fn write(&mut self, address: Address, width: BusWidth, value: Bitfield) {
        if let Some(expected) = self.write_inputs.pop() {
            assert_eq!(
                expected,
                (address, width, value),
                "Incorrect inputs: {:#x}, {}, {}. Expected: {:#x}, {}, {}",
                address.0,
                width.bytes(),
                value,
                expected.0.0,
                expected.1.bytes(),
                expected.2
            );
        } else {
            panic!("More calls to write than Expected!");
        }
    }
}
impl Drop for TestBus {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.read_inputs.is_empty(), "Insufficient Reads");
            assert!(self.write_inputs.is_empty(), "Insufficient Reads");
        }
    }
}

use crate::gba::{
    bitmanip::Bitfield,
    memory::{Bus, BusWidth, bus::Address},
};

pub(crate) struct TestBus {
    read_inputs: Vec<(Address, BusWidth)>,
    read_outputs: Vec<Bitfield>,
    write_inputs: Vec<(Address, BusWidth, Bitfield)>,
}

impl TestBus {
    #[cfg_attr(not(test), expect(dead_code, reason = "Testing purposes"))]
    pub(crate) fn new(
        read_inputs: Vec<(Address, BusWidth)>,
        read_outputs: Vec<Bitfield>,
        write_inputs: Vec<(Address, BusWidth, Bitfield)>,
    ) -> Self {
        assert_eq!(
            read_inputs.len(),
            read_outputs.len(),
            "Each read input must have an output"
        );
        let read_inputs = read_inputs.into_iter().rev().collect();
        let read_outputs = read_outputs.into_iter().rev().collect();
        let write_inputs = write_inputs.into_iter().rev().collect();
        Self {
            read_inputs,
            read_outputs,
            write_inputs,
        }
    }
}

impl Bus for TestBus {
    fn read(&mut self, address: Address, width: BusWidth) -> Bitfield {
        if let Some((expected_address, expected_width)) = self.read_inputs.pop() {
            if expected_address == address && expected_width == width {
                self.read_outputs.pop().expect("this cannot occur")
            } else {
                panic!(
                    "Incorrect inputs: {:#x}, {}. Expected: {:#x}, {}",
                    address.0,
                    width.bytes(),
                    expected_address.0,
                    width.bytes()
                )
            }
        } else {
            panic!("More calls to read than Expected!");
        }
    }

    fn write(&mut self, address: Address, width: BusWidth, value: Bitfield) {
        if let Some((expected_address, expected_width, expected_value)) = self.write_inputs.pop() {
            if expected_address == address && expected_width == width && expected_value == value {
            } else {
                panic!(
                    "Incorrect inputs: {:#x}, {}, {}. Expected: {:#x}, {}, {}",
                    address.0,
                    width.bytes(),
                    value,
                    expected_address.0,
                    expected_width.bytes(),
                    expected_value
                )
            }
        } else {
            panic!("More calls to read than Expected!");
        }
    }
}

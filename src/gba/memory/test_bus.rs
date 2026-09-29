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
            assert!(
                expected_address == address && expected_width == width,
                "Incorrect read inputs: {:#x}, {}. Expected: {:#x}, {}",
                address.0,
                width.bytes(),
                expected_address.0,
                expected_width.bytes()
            );
            output
        } else {
            panic!("More calls to read than Expected!");
        }
    }

    fn write(&mut self, address: Address, width: BusWidth, value: Bitfield) {
        if let Some(expected) = self.write_inputs.pop() {
            assert_eq!(
                expected,
                (address, width, value),
                "Incorrect write inputs: {:#x}, {}, {:#x}. Expected: {:#x}, {}, {:#x}",
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
            assert!(
                self.read_inputs.is_empty(),
                "Insufficient Reads: {:?}",
                self.read_inputs
            );
            assert!(
                self.write_inputs.is_empty(),
                "Insufficient Writes: {:?}",
                self.write_inputs
            );
        }
    }
}

mod tests {
    use super::*;

    #[test]
    #[should_panic = "Incorrect write inputs"]
    fn panics_on_wrong_address_to_write() {
        let read_inputs = vec![];
        let write_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let mut bus = TestBus::new(read_inputs, write_inputs);

        bus.write(Address(1), BusWidth::B8, Bitfield::new(0));
    }

    #[test]
    #[should_panic = "Incorrect write inputs"]
    fn panics_on_wrong_width_to_write() {
        let read_inputs = vec![];
        let write_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let mut bus = TestBus::new(read_inputs, write_inputs);

        bus.write(Address(0), BusWidth::B16, Bitfield::new(0));
    }

    #[test]
    #[should_panic = "Incorrect write inputs"]
    fn panics_on_wrong_value_to_write() {
        let read_inputs = vec![];
        let write_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let mut bus = TestBus::new(read_inputs, write_inputs);

        bus.write(Address(0), BusWidth::B8, Bitfield::new(1));
    }

    #[test]
    #[should_panic = "Incorrect read inputs"]
    fn panics_on_wrong_address_to_read() {
        let read_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let write_inputs = vec![];
        let mut bus = TestBus::new(read_inputs, write_inputs);

        bus.read(Address(1), BusWidth::B8);
    }

    #[test]
    #[should_panic = "Incorrect read inputs"]
    fn panics_on_wrong_width_to_read() {
        let read_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let write_inputs = vec![];
        let mut bus = TestBus::new(read_inputs, write_inputs);

        bus.read(Address(0), BusWidth::B16);
    }

    #[test]
    #[should_panic = "More calls to write than Expected!"]
    fn panics_on_too_many_calls_to_write() {
        let read_inputs = vec![];
        let write_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let mut bus = TestBus::new(read_inputs, write_inputs);

        bus.write(Address(0), BusWidth::B8, Bitfield::new(0));
        bus.write(Address(0), BusWidth::B8, Bitfield::new(0));
    }

    #[test]
    #[should_panic = "More calls to read than Expected!"]
    fn panics_on_too_many_calls_to_read() {
        let read_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let write_inputs = vec![];
        let mut bus = TestBus::new(read_inputs, write_inputs);

        let result = bus.read(Address(0), BusWidth::B8);
        assert_eq!(result, Bitfield::new(0));
        _ = bus.read(Address(0), BusWidth::B8);
    }

    #[test]
    #[should_panic = "Insufficient Writes:"]
    fn panics_on_too_few_calls_to_write() {
        let read_inputs = vec![];
        let write_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let _bus = TestBus::new(read_inputs, write_inputs);
    }

    #[test]
    #[should_panic = "Insufficient Reads:"]
    fn panics_on_too_few_calls_to_read() {
        let read_inputs = vec![(Address(0), BusWidth::B8, Bitfield::new(0))];
        let write_inputs = vec![];
        let _bus = TestBus::new(read_inputs, write_inputs);
    }

    #[test]
    fn correct_use() {
        let read_output_1 = Bitfield::new(1);
        let read_output_2 = Bitfield::new(2);
        let read_val_1 = (Address(1), BusWidth::B8, read_output_1);
        let read_val_2 = (Address(2), BusWidth::B16, read_output_2);
        let write_val = (Address(3), BusWidth::B32, Bitfield::new(3));
        let read_inputs = vec![read_val_1, read_val_2];
        let write_inputs = vec![write_val];
        let mut bus = TestBus::new(read_inputs, write_inputs);

        let read_result_1 = bus.read(read_val_1.0, read_val_1.1);
        assert_eq!(read_result_1, read_val_1.2);

        bus.write(write_val.0, write_val.1, write_val.2);

        let read_result_2 = bus.read(read_val_2.0, read_val_2.1);
        assert_eq!(read_result_2, read_val_2.2);
    }
}

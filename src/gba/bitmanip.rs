#[derive(Debug, Clone, Copy)]
pub(crate) struct Bitfield(u32);

impl Bitfield {
    pub(crate) fn new(input: u32) -> Self {
        Self(input)
    }
    pub(crate) fn get_field(self, index: u32) -> bool {
        debug_assert!(index < 32);
        self.0 & (1 << index) != 0
    }
    pub(crate) fn set_field(&mut self, index: u32, value: bool) {
        let val = u32::from(value);
        debug_assert!(index < 32);
        self.0 = (self.0 & !(1 << index)) | (val << index);
    }

    pub(crate) fn get_range(self, start: u32, end: u32) -> u32 {
        debug_assert!(end < 32);
        debug_assert!(start < end);
        let mut subfield = Self::new(0);
        for i in start..=end {
            subfield.set_field(i, self.get_field(i));
        }
        subfield.raw().strict_shr(start)
    }
    fn raw(self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_arbitrary_bit() {
        for i in 0..32 {
            let bitfield = Bitfield::new(1 << i);

            assert!(bitfield.get_field(i));
        }
    }
    #[test]
    fn sets_arbitrary_bit() {
        for i in 0..32 {
            let mut bitfield = Bitfield::new(0);

            bitfield.set_field(i, true);

            assert!(bitfield.get_field(i));
        }
    }
    #[test]
    fn unsets_arbitrary_bit() {
        for i in 0..32 {
            let mut bitfield = Bitfield::new(u32::MAX);

            bitfield.set_field(i, false);

            assert!(!bitfield.get_field(i));
        }
    }

    #[test]
    fn gets_range() {
        let bitfield = Bitfield::new(u32::MAX);

        let result = bitfield.get_range(0, 3);

        assert_eq!(result, 15);
    }

    #[test]
    fn gets_range_and_shifts_to_base() {
        let bitfield = Bitfield::new(u32::MAX);

        let result = bitfield.get_range(6, 9);

        assert_eq!(result, 15);
    }
}

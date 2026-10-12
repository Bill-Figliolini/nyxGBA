use crate::gba::bitmanip::Bitfield;

#[derive(Debug, Clone, Copy)]
pub(in crate::gba::cpu) struct CurrentProgramStatusRegister(Bitfield);

impl CurrentProgramStatusRegister {
    pub(crate) fn new() -> Self {
        Self(Bitfield::new(0))
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "Testing purposes"))]
    #[allow(clippy::fn_params_excessive_bools, reason = "Testing purposes")]
    pub(crate) fn with_values(negative: bool, zero: bool, carry: bool, overflow: bool) -> Self {
        let mut flags = Self(Bitfield::new(0));
        flags.set_negative(negative);
        flags.set_zero(zero);
        flags.set_carry(carry);
        flags.set_overflow(overflow);
        flags
    }
    pub(crate) fn reset(&mut self) {
        self.0.set_value(0);
    }

    // order, from highest bit to lowest:
    // negative
    // zero
    // carry
    // overflow
    pub(crate) fn negative(self) -> bool {
        self.0.bit(31)
    }
    pub(crate) fn set_negative(&mut self, val: bool) {
        self.0.set_bit(31, val);
    }
    pub(crate) fn zero(self) -> bool {
        self.0.bit(30)
    }
    pub(crate) fn set_zero(&mut self, val: bool) {
        self.0.set_bit(30, val);
    }
    pub(crate) fn set_nz(&mut self, val: u32) {
        self.set_negative(val >> 31 == 1);
        self.set_zero(val == 0);
    }
    pub(crate) fn carry(self) -> bool {
        self.0.bit(29)
    }
    pub(crate) fn set_carry(&mut self, val: bool) {
        self.0.set_bit(29, val);
    }
    pub(crate) fn overflow(self) -> bool {
        self.0.bit(28)
    }
    pub(crate) fn set_overflow(&mut self, val: bool) {
        self.0.set_bit(28, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod flags {
        use super::*;
        #[test]
        fn negative_is_correct_bit() {
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_negative(true);

            assert!(flags.negative());
        }
        #[test]
        fn zero_is_correct_bit() {
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_zero(true);

            assert!(flags.zero());
        }
        #[test]
        fn carry_is_correct_bit() {
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_carry(true);

            assert!(flags.carry());
        }
        #[test]
        fn overflow_is_correct_bit() {
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_overflow(true);

            assert!(flags.overflow());
        }
    }
    mod nz_logic {
        use super::*;
        #[test]
        fn sets_on_zero() {
            let val = 0;
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_nz(val);

            assert!(flags.zero());
            assert!(!flags.negative());
        }
        #[test]
        fn sets_on_smallest_neg() {
            let val = 0x8000_0000;
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_nz(val);

            assert!(!flags.zero());
            assert!(flags.negative());
        }
        #[test]
        fn does_not_set_on_largest_positive() {
            let val = 0x7FFF_FFFF;
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_nz(val);

            assert!(!flags.zero());
            assert!(!flags.negative());
        }
        #[test]
        fn clears_existing_flags() {
            let val = 1;
            let mut flags = CurrentProgramStatusRegister::with_values(true, true, false, false);

            flags.set_nz(val);

            assert!(!flags.zero());
            assert!(!flags.negative());
        }
    }
}

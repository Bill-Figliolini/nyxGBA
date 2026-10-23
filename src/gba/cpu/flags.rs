use crate::gba::bitmanip::Bitfield;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(in crate::gba::cpu) struct CurrentProgramStatusRegister(Bitfield);

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct FlagApplication {
    pub zero: bool,
    pub negative: bool,
    pub carry: Option<bool>,
    pub overflow: Option<bool>,
}
impl FlagApplication {
    pub(crate) fn new(value: u32, carry: Option<bool>, overflow: Option<bool>) -> Self {
        let zero = value == 0;
        let negative = value >> 31 == 1;
        Self {
            zero,
            negative,
            carry,
            overflow,
        }
    }
}

impl CurrentProgramStatusRegister {
    pub(crate) fn new() -> Self {
        Self(Bitfield::new(0))
    }

    pub(crate) fn reset(&mut self) {
        self.0.set_value(0);
    }

    pub(crate) fn apply(&mut self, flags: FlagApplication) {
        self.set_zero(flags.zero);
        self.set_negative(flags.negative);
        if let Some(carry) = flags.carry {
            self.set_carry(carry);
        }
        if let Some(overflow) = flags.overflow {
            self.set_overflow(overflow);
        }
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
#[derive(Debug)]
pub(in crate::gba::cpu) struct CPSRBuilder(CurrentProgramStatusRegister);

#[cfg(test)]
impl CurrentProgramStatusRegister {
    pub(crate) fn builder() -> CPSRBuilder {
        CPSRBuilder(CurrentProgramStatusRegister::new())
    }
}

#[cfg(test)]
impl CPSRBuilder {
    pub(crate) fn negative(mut self) -> Self {
        self.0.set_negative(true);
        self
    }
    pub(crate) fn zero(mut self) -> Self {
        self.0.set_zero(true);
        self
    }
    pub(crate) fn carry(mut self) -> Self {
        self.0.set_carry(true);
        self
    }
    pub(crate) fn overflow(mut self) -> Self {
        self.0.set_overflow(true);
        self
    }
    pub(crate) fn build(self) -> CurrentProgramStatusRegister {
        self.0
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

            let flags = FlagApplication::new(val, None, None);

            assert!(flags.zero);
            assert!(!flags.negative);
        }
        #[test]
        fn sets_on_smallest_neg() {
            let val = 0x8000_0000;

            let flags = FlagApplication::new(val, None, None);

            assert!(!flags.zero);
            assert!(flags.negative);
        }
        #[test]
        fn does_not_set_on_largest_positive() {
            let val = 0x7FFF_FFFF;

            let flags = FlagApplication::new(val, None, None);

            assert!(!flags.zero);
            assert!(!flags.negative);
        }
        #[test]
        fn clears_existing_flags() {
            let val = 1;

            let flags = FlagApplication::new(val, None, None);

            assert!(!flags.zero);
            assert!(!flags.negative);
        }
    }
    mod cv_apply {
        use super::*;

        #[test]
        fn carry_preserved_on_none_true() {
            let mut cpsr = CurrentProgramStatusRegister::builder().carry().build();
            let flags = FlagApplication::new(0, None, Some(true));

            cpsr.apply(flags);

            assert!(cpsr.zero());
            assert!(!cpsr.negative());
            assert!(cpsr.carry());
            assert!(cpsr.overflow());
        }

        #[test]
        fn carry_preserved_on_none_false() {
            let mut cpsr = CurrentProgramStatusRegister::builder().build();
            let flags = FlagApplication::new(0, None, Some(false));

            cpsr.apply(flags);

            assert!(cpsr.zero());
            assert!(!cpsr.negative());
            assert!(!cpsr.carry());
            assert!(!cpsr.overflow());
        }
        #[test]
        fn carry_overwritten_on_some_true() {
            let mut cpsr = CurrentProgramStatusRegister::builder().build();
            let flags = FlagApplication::new(0, Some(true), Some(true));

            cpsr.apply(flags);

            assert!(cpsr.zero());
            assert!(!cpsr.negative());
            assert!(cpsr.carry());
            assert!(cpsr.overflow());
        }

        #[test]
        fn carry_overwritten_on_some_false() {
            let mut cpsr = CurrentProgramStatusRegister::builder().carry().build();
            let flags = FlagApplication::new(0, Some(false), Some(false));

            cpsr.apply(flags);

            assert!(cpsr.zero());
            assert!(!cpsr.negative());
            assert!(!cpsr.carry());
            assert!(!cpsr.overflow());
        }
        #[test]
        fn overflow_preserved_on_none_true() {
            let mut cpsr = CurrentProgramStatusRegister::builder().overflow().build();
            let flags = FlagApplication::new(0, Some(true), None);

            cpsr.apply(flags);

            assert!(cpsr.zero());
            assert!(!cpsr.negative());
            assert!(cpsr.carry());
            assert!(cpsr.overflow());
        }

        #[test]
        fn overflow_preserved_on_none_false() {
            let mut cpsr = CurrentProgramStatusRegister::builder().build();
            let flags = FlagApplication::new(0, Some(false), None);

            cpsr.apply(flags);

            assert!(cpsr.zero());
            assert!(!cpsr.negative());
            assert!(!cpsr.carry());
            assert!(!cpsr.overflow());
        }
        #[test]
        fn overflow_overwritten_on_some_true() {
            let mut cpsr = CurrentProgramStatusRegister::builder().build();
            let flags = FlagApplication::new(0, Some(true), Some(true));

            cpsr.apply(flags);

            assert!(cpsr.zero());
            assert!(!cpsr.negative());
            assert!(cpsr.carry());
            assert!(cpsr.overflow());
        }

        #[test]
        fn overflow_overwritten_on_some_false() {
            let mut cpsr = CurrentProgramStatusRegister::builder().overflow().build();
            let flags = FlagApplication::new(0, Some(false), Some(false));

            cpsr.apply(flags);

            assert!(cpsr.zero());
            assert!(!cpsr.negative());
            assert!(!cpsr.carry());
            assert!(!cpsr.overflow());
        }
    }
}

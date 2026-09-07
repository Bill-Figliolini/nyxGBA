#[derive(Debug, Clone, Copy)]
pub(in crate::cpu) struct CurrentProgramStatusRegister(u32);

#[allow(dead_code, reason = "To be used later")]
impl CurrentProgramStatusRegister {
    pub(crate) fn new() -> Self {
        Self(0)
    }
    //Resets the arithmetic flags before operations
    // order, from highest bit to lowest:
    // signed
    // zero
    // carry
    // overflow
    pub(crate) fn clear_nzcv(&mut self) {
        self.0 &= !(0b1111 << 28);
    }
    pub(crate) fn get_signed_flag(self) -> bool {
        self.0 & (1 << 31) != 0
    }
    pub(crate) fn set_signed_flag(&mut self) {
        self.0 |= 1 << 31;
    }
    pub(crate) fn get_zero_flag(self) -> bool {
        self.0 & (1 << 30) != 0
    }
    pub(crate) fn set_zero_flag(&mut self) {
        self.0 |= 1 << 30;
    }
    pub(crate) fn get_carry_flag(self) -> bool {
        self.0 & (1 << 29) != 0
    }
    pub(crate) fn set_carry_flag(&mut self) {
        self.0 |= 1 << 29;
    }
    pub(crate) fn get_overflow_flag(self) -> bool {
        self.0 & (1 << 28) != 0
    }
    pub(crate) fn set_overflow_flag(&mut self) {
        self.0 |= 1 << 28;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod flags {
        use super::*;
        #[test]
        fn signed_is_correct_bit() {
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_signed_flag();

            assert_eq!(flags.0, 1 << 31);
            assert!(flags.get_signed_flag());
        }
        #[test]
        fn zero_is_correct_bit() {
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_zero_flag();

            assert_eq!(flags.0, 1 << 30);
            assert!(flags.get_zero_flag());
        }
        #[test]
        fn carry_is_correct_bit() {
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_carry_flag();

            assert_eq!(flags.0, 1 << 29);
            assert!(flags.get_carry_flag());
        }
        #[test]
        fn overflow_is_correct_bit() {
            let mut flags = CurrentProgramStatusRegister::new();

            flags.set_overflow_flag();

            assert_eq!(flags.0, 1 << 28);
            assert!(flags.get_overflow_flag());
        }
        #[test]
        fn clear_clears_all_flags() {
            let mut flags = CurrentProgramStatusRegister::new();
            flags.set_carry_flag();
            flags.set_overflow_flag();
            flags.set_zero_flag();
            flags.set_signed_flag();

            flags.clear_nzcv();

            assert_eq!(flags.0, 0);
        }
    }
}

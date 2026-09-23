use crate::cpu::Register;

#[derive(Debug)]
#[expect(dead_code, reason = "To be used later")]
pub(crate) struct ArmCommand {
    pub condition: ArmCondition,
    pub op_code: ArmOpCode,
    pub set_flag: bool,
    pub destination_reg: Register,
    pub read_reg: Register,
    pub source: SourceOperand,
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
#[cfg_attr(not(test), expect(dead_code, reason = "To be used later"))]
pub(crate) enum ArmCondition {
    Equal,
    NotEqual,
    CarrySet,
    CarryCleared,
    Minus,
    Plus,
    SignedOverflow,
    NoSignedOverflow,
    UnsignedHigher,
    UnsignedLowerOrSame,
    SignedGreaterEq,
    SignedLesser,
    SignedGreater,
    SignedLesserEq,
    Always,
    Never,
}

impl ArmCondition {
    pub(crate) fn new(input: u32) -> Self {
        debug_assert!(input < 0x10);
        #[allow(
            clippy::as_conversions,
            reason = "input is restricted to less than 0x10, which fits in a u8"
        )]
        #[allow(
            clippy::cast_possible_truncation,
            reason = "input is restricted to less than 0x10, which fits in a u8"
        )]
        // SAFETY:
        // input must be less than 0x10
        //can now be infallibly converted to Armcondition
        unsafe {
            std::mem::transmute(input as u8)
        }
    }
}

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "To be used later"))]
pub(crate) enum ArmOpCode {
    Mov,
    Add,
}

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "To be used later"))]
pub(crate) enum SourceOperand {
    Immediate(u32),
    Register(Register),
}

#[cfg(test)]
mod tests {
    use super::*;
    mod condition {
        use super::*;
        #[test]
        fn accepts_values_from_0_to_f() {
            let results = vec![
                ArmCondition::Equal,
                ArmCondition::NotEqual,
                ArmCondition::CarrySet,
                ArmCondition::CarryCleared,
                ArmCondition::Minus,
                ArmCondition::Plus,
                ArmCondition::SignedOverflow,
                ArmCondition::NoSignedOverflow,
                ArmCondition::UnsignedHigher,
                ArmCondition::UnsignedLowerOrSame,
                ArmCondition::SignedGreaterEq,
                ArmCondition::SignedLesser,
                ArmCondition::SignedGreater,
                ArmCondition::SignedLesserEq,
                ArmCondition::Always,
                ArmCondition::Never,
            ];
            let mut results_iter = results.into_iter();
            for i in 0..0x10 {
                assert_eq!(ArmCondition::new(i), results_iter.next().unwrap());
            }
            assert_eq!(results_iter.next(), None);
        }

        #[test]
        #[should_panic = "assertion failed: input < 0x10"]
        fn debug_panics_on_value_outside_range() {
            ArmCondition::new(0x10);
        }
    }
}

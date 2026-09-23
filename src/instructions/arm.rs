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
        //Should not allow for more than the 16 available options, and
        // assert will provide warning.
        debug_assert!(input < 0x10);

        match input & 0xF {
            0x0 => ArmCondition::Equal,
            0x1 => ArmCondition::NotEqual,
            0x2 => ArmCondition::CarrySet,
            0x3 => ArmCondition::CarryCleared,
            0x4 => ArmCondition::Minus,
            0x5 => ArmCondition::Plus,
            0x6 => ArmCondition::SignedOverflow,
            0x7 => ArmCondition::NoSignedOverflow,
            0x8 => ArmCondition::UnsignedHigher,
            0x9 => ArmCondition::UnsignedLowerOrSame,
            0xA => ArmCondition::SignedGreaterEq,
            0xB => ArmCondition::SignedLesser,
            0xC => ArmCondition::SignedGreater,
            0xD => ArmCondition::SignedLesserEq,
            0xE => ArmCondition::Always,
            0xF => ArmCondition::Never,
            _ => unreachable!(),
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
    }
}

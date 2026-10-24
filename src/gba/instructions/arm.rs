use crate::gba::{bitmanip::Bitfield, cpu::Register};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArmInstruction {
    pub condition: ArmCondition,
    pub kind: ArmKind,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ArmKind {
    DataProcessing(DataProcessing),
    Unimplemented(Bitfield),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DataProcessing {
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
    SignedLess,
    SignedGreater,
    SignedLessEq,
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
            0xB => ArmCondition::SignedLess,
            0xC => ArmCondition::SignedGreater,
            0xD => ArmCondition::SignedLessEq,
            0xE => ArmCondition::Always,
            0xF => ArmCondition::Never,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ArmOpCode {
    Mov,
    Add,
    Unimplemented(Bitfield),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SourceOperand {
    Immediate {
        value: u32,
        shift: u32,
    },
    Register {
        register: Register,
        shift: Option<Shift>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Shift {
    pub kind: ShiftKind,
    pub amount: ShiftAmount,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ShiftKind {
    LogicalShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
    RotateRight,
    RotateRightExtend,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ShiftAmount {
    //Shift Immediates must be constrained to 32 max
    Immediate(u32),
    Register(Register),
}

impl ShiftKind {
    fn from_bits(input: u32) -> Self {
        match input {
            0 => Self::LogicalShiftLeft,
            1 => Self::LogicalShiftRight,
            2 => Self::ArithmeticShiftRight,
            3 => Self::RotateRight,
            _ => unreachable!(),
        }
    }
}

impl Shift {
    #[expect(clippy::todo, reason = "Will be finished next commit")]
    pub(crate) fn shift(self, input: u32, value: u32, _carry: bool) -> (u32, bool) {
        match self.kind {
            ShiftKind::LogicalShiftLeft => todo!(),
            ShiftKind::LogicalShiftRight => todo!(),
            ShiftKind::ArithmeticShiftRight => todo!(),
            ShiftKind::RotateRight => {
                let result = input.rotate_right(value);
                let carry = result >> 31 == 1;
                (result, carry)
            }
            ShiftKind::RotateRightExtend => todo!(),
        }
    }
}

impl SourceOperand {
    pub(in crate::gba::instructions) fn from_bits(input: Bitfield) -> Self {
        if input.bit(25) {
            let shift = input.bits(8, 4).strict_mul(2);
            let value = input.bits(0, 8);
            Self::Immediate { value, shift }
        } else {
            let register = Register::from_bits(input, 0);
            let kind = ShiftKind::from_bits(input.bits(5, 2));
            let shift = if input.bit(4) {
                Some(Shift {
                    kind,
                    amount: ShiftAmount::Register(Register::from_bits(input, 8)),
                })
            } else {
                match (kind, input.bits(7, 5)) {
                    (ShiftKind::LogicalShiftLeft, 0) => None,
                    (ShiftKind::LogicalShiftRight | ShiftKind::ArithmeticShiftRight, 0) => {
                        Some(Shift {
                            kind,
                            amount: ShiftAmount::Immediate(32),
                        })
                    }
                    (ShiftKind::RotateRight, 0) => Some(Shift {
                        kind: ShiftKind::RotateRightExtend,
                        amount: ShiftAmount::Immediate(1),
                    }),
                    (kind, amount) => {
                        debug_assert!(
                            0 < amount,
                            "ShiftAmount::Immediate must be above 0. Only violable if ShiftKind::from_bits starts producing RotateRightExtend"
                        );
                        Some(Shift {
                            kind,
                            amount: ShiftAmount::Immediate(amount),
                        })
                    }
                }
            };
            Self::Register { register, shift }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod condition {
        use super::*;
        const ALL: [ArmCondition; 16] = [
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
            ArmCondition::SignedLess,
            ArmCondition::SignedGreater,
            ArmCondition::SignedLessEq,
            ArmCondition::Always,
            ArmCondition::Never,
        ];
        #[test]
        fn accepts_values_from_0_to_f() {
            let results = ALL;
            let mut results_iter = results.into_iter();
            for i in 0..0x10 {
                assert_eq!(ArmCondition::new(i), results_iter.next().unwrap());
            }
            assert_eq!(results_iter.next(), None);
        }
    }
    mod source_operand {
        use super::*;
        mod immediate {
            use super::*;
            #[test]
            fn read_immediate_unshifted() {
                let input = Bitfield::new(0b0000_0010_0000_0000_0000_0000_1111_1111);
                let expected_result = SourceOperand::Immediate {
                    value: 0b1111_1111,
                    shift: 0,
                };

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }
            #[test]
            fn read_immediate_shifted() {
                let input = Bitfield::new(0b0000_0010_0000_0000_0000_0001_1111_1111);
                let expected_result = SourceOperand::Immediate {
                    value: 0b0000_0000_0000_0000_0000_0000_1111_1111,
                    shift: 2,
                };

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }
        }
        mod register {
            use super::*;
            #[test]
            fn produces_all_registers() {
                let mut results = Register::ALL.into_iter();

                for i in 0..0x10 {
                    let expected_reg = results.next().unwrap();
                    let actual_reg = SourceOperand::from_bits(Bitfield::new(i));

                    assert_eq!(
                        actual_reg,
                        SourceOperand::Register {
                            register: expected_reg,
                            shift: None
                        }
                    );
                }
                assert!(results.next().is_none());
            }
            mod barrel_shifter {
                #[allow(unused_imports, reason = "Will be implemented next commit")]
                use super::*;
                //TODO: Implement tests for Barrel shifter, Covering parsing and
                // behavior
            }
        }
    }
}

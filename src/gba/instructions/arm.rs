use crate::gba::{
    bitmanip::{Bitfield, bit},
    cpu::Register,
};

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
    //Shift Immediates are restricted to 5 bits, and can not exceed 32
    Immediate(u32),
    Register(Register),
}

impl ShiftKind {
    fn from_bits(input: Bitfield, start: u32) -> Self {
        let bits = input.bits(start, 2);
        match bits {
            0 => Self::LogicalShiftLeft,
            1 => Self::LogicalShiftRight,
            2 => Self::ArithmeticShiftRight,
            3 => Self::RotateRight,
            _ => unreachable!(),
        }
    }
}

impl Shift {
    pub(crate) fn shift(self, input: u32, amount: u32, carry: bool) -> (u32, bool) {
        if amount == 0 {
            return (input, carry);
        }
        match self.kind {
            ShiftKind::LogicalShiftLeft => match input.checked_shl(amount) {
                // Strict sub here can not panic, shl will return none if 32 or greater
                Some(result) => (result, bit(input, 32u32.strict_sub(amount))),
                None if amount == 32 => (0, bit(input, 0)),
                None => (0, false),
            },
            ShiftKind::LogicalShiftRight => match input.checked_shr(amount) {
                // amount is guaranteed to be 1 or greater, so no panic can occur
                Some(result) => (result, bit(input, amount.strict_sub(1))),
                None if amount == 32 => (0, bit(input, 31)),
                None => (0, false),
            },
            ShiftKind::ArithmeticShiftRight => match input.cast_signed().checked_shr(amount) {
                // amount is guaranteed to be 1 or greater, so no panic can occur
                Some(result) => (result.cast_unsigned(), bit(input, amount.strict_sub(1))),
                None => (if bit(input, 31) { u32::MAX } else { 0 }, bit(input, 31)),
            },
            ShiftKind::RotateRight => {
                let result = input.rotate_right(amount);
                (result, bit(result, 31))
            }
            ShiftKind::RotateRightExtend => {
                ((u32::from(carry) << 31) | (input >> 1), bit(input, 0))
            }
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
            let kind = ShiftKind::from_bits(input, 5);
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
        }
    }
    mod barrel_shifter {
        use super::*;

        fn operand(register: Register, shift: Option<Shift>) -> SourceOperand {
            SourceOperand::Register { register, shift }
        }
        mod logical_shift_left {
            use super::*;
            const KIND: ShiftKind = ShiftKind::LogicalShiftLeft;

            fn shifted_operand(register: Register, amount: ShiftAmount) -> SourceOperand {
                SourceOperand::Register {
                    register,
                    shift: Some(Shift { kind: KIND, amount }),
                }
            }

            #[test]
            fn zero_shift_produces_none() {
                let input = Bitfield::new(0b0000_0000_0000_0000_0000_0000_0000_0000);
                let expected_shift = None;
                let expected_register = Register::R0;
                let expected_result = operand(expected_register, expected_shift);

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }

            #[test]
            fn shift_parse_immediate() {
                let input = Bitfield::new(0b0000_0000_0000_0000_0000_1111_1000_0000);
                let expected_amount = ShiftAmount::Immediate(0b11111);
                let expected_register = Register::R0;
                let expected_result = shifted_operand(expected_register, expected_amount);

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }

            #[test]
            fn shift_parse_register() {
                let mut input = Bitfield::new(0b0000_0000_0000_0000_0000_0000_0001_0000);
                for (value, register) in (0u32..).zip(Register::ALL) {
                    input.set_bits(8, 4, value);
                    let expected_amount = ShiftAmount::Register(register);
                    let expected_register = Register::R0;
                    let expected_result = shifted_operand(expected_register, expected_amount);

                    let actual_result = SourceOperand::from_bits(input);

                    assert_eq!(actual_result, expected_result);
                }
            }

            #[test]
            #[ignore = "not yet written"]
            fn minimum_shift() {}

            #[test]
            #[ignore = "not yet written"]
            fn shift_32() {}

            #[test]
            #[ignore = "not yet written"]
            fn shift_greater_than_32() {}
        }
        mod logical_shift_right {
            use super::*;
            const KIND: ShiftKind = ShiftKind::LogicalShiftRight;

            fn shifted_operand(register: Register, amount: ShiftAmount) -> SourceOperand {
                SourceOperand::Register {
                    register,
                    shift: Some(Shift { kind: KIND, amount }),
                }
            }

            #[test]
            fn zero_shift_produces_immediate() {
                let input = Bitfield::new(0b0000_0000_0000_0000_0000_0000_0010_0000);
                let expected_amount = ShiftAmount::Immediate(32);
                let expected_register = Register::R0;
                let expected_result = shifted_operand(expected_register, expected_amount);

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }

            #[test]
            fn shift_parse_immediate() {
                let input = Bitfield::new(0b0000_0000_0000_0000_0000_1111_1010_0000);
                let expected_amount = ShiftAmount::Immediate(0b11111);
                let expected_register = Register::R0;
                let expected_result = shifted_operand(expected_register, expected_amount);

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }

            #[test]
            fn shift_parse_register() {
                let mut input = Bitfield::new(0b0000_0000_0000_0000_0000_0000_0011_0000);
                for (value, register) in (0u32..).zip(Register::ALL) {
                    input.set_bits(8, 4, value);
                    let expected_amount = ShiftAmount::Register(register);
                    let expected_register = Register::R0;
                    let expected_result = shifted_operand(expected_register, expected_amount);

                    let actual_result = SourceOperand::from_bits(input);

                    assert_eq!(actual_result, expected_result);
                }
            }
            #[test]
            #[ignore = "not yet written"]
            fn minimum_shift() {}

            #[test]
            #[ignore = "not yet written"]
            fn shift_32() {}

            #[test]
            #[ignore = "not yet written"]
            fn shift_greater_than_32() {}
        }
        mod arithmetic_shift_right {
            use super::*;
            const KIND: ShiftKind = ShiftKind::ArithmeticShiftRight;

            fn shifted_operand(register: Register, amount: ShiftAmount) -> SourceOperand {
                SourceOperand::Register {
                    register,
                    shift: Some(Shift { kind: KIND, amount }),
                }
            }

            #[test]
            fn zero_shift_produces_immediate() {
                let input = Bitfield::new(0b0000_0000_0000_0000_0000_0000_0100_0000);
                let expected_amount = ShiftAmount::Immediate(32);
                let expected_register = Register::R0;
                let expected_result = shifted_operand(expected_register, expected_amount);

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }

            #[test]
            fn shift_parse_immediate() {
                let input = Bitfield::new(0b0000_0000_0000_0000_0000_1111_1100_0000);
                let expected_amount = ShiftAmount::Immediate(0b11111);
                let expected_register = Register::R0;
                let expected_result = shifted_operand(expected_register, expected_amount);

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }

            #[test]
            fn shift_parse_register() {
                let mut input = Bitfield::new(0b0000_0000_0000_0000_0000_0000_0101_0000);
                for (value, register) in (0u32..).zip(Register::ALL) {
                    input.set_bits(8, 4, value);
                    let expected_amount = ShiftAmount::Register(register);
                    let expected_register = Register::R0;
                    let expected_result = shifted_operand(expected_register, expected_amount);

                    let actual_result = SourceOperand::from_bits(input);

                    assert_eq!(actual_result, expected_result);
                }
            }

            #[test]
            #[ignore = "not yet written"]
            fn minimum_shift() {}

            #[test]
            #[ignore = "not yet written"]
            fn shift_32() {}

            #[test]
            #[ignore = "not yet written"]
            fn shift_greater_than_32() {}
        }
        mod rotate_right {
            use super::*;
            const KIND: ShiftKind = ShiftKind::RotateRight;

            fn shifted_operand(register: Register, amount: ShiftAmount) -> SourceOperand {
                SourceOperand::Register {
                    register,
                    shift: Some(Shift { kind: KIND, amount }),
                }
            }

            #[test]
            fn zero_shift_produces_extend() {
                let input = Bitfield::new(0b0000_0000_0000_0000_0000_0000_0110_0000);
                let expected_shift = Some(Shift {
                    kind: ShiftKind::RotateRightExtend,
                    amount: ShiftAmount::Immediate(1),
                });
                let expected_register = Register::R0;
                let expected_result = operand(expected_register, expected_shift);

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }

            #[test]
            fn shift_parse_immediate() {
                let input = Bitfield::new(0b0000_0000_0000_0000_0000_1111_1110_0000);
                let expected_amount = ShiftAmount::Immediate(0b11111);
                let expected_register = Register::R0;
                let expected_result = shifted_operand(expected_register, expected_amount);

                let actual_result = SourceOperand::from_bits(input);

                assert_eq!(actual_result, expected_result);
            }

            #[test]
            fn shift_parse_register() {
                let mut input = Bitfield::new(0b0000_0000_0000_0000_0000_0000_0111_0000);
                for (value, register) in (0u32..).zip(Register::ALL) {
                    input.set_bits(8, 4, value);
                    let expected_amount = ShiftAmount::Register(register);
                    let expected_register = Register::R0;
                    let expected_result = shifted_operand(expected_register, expected_amount);

                    let actual_result = SourceOperand::from_bits(input);

                    assert_eq!(actual_result, expected_result);
                }
            }

            #[test]
            #[ignore = "not yet written"]
            fn minimum_shift() {}

            #[test]
            #[ignore = "not yet written"]
            fn shift_32() {}

            #[test]
            #[ignore = "not yet written"]
            fn shift_greater_than_32() {}
        }
        mod rotate_right_extend {
            //const KIND: ShiftKind = ShiftKind::RotateRightExtend;
        }
    }
}

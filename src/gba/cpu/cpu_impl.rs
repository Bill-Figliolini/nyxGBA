use crate::gba::{
    bitmanip::Bitfield,
    clock::Time,
    cpu::{
        Register, arm_exec::compute_arm_actions, flags::CurrentProgramStatusRegister,
        registers::Registers,
    },
    instructions::{ArmCondition, ArmInstruction, SourceOperand, parse_arm},
    memory::{Address, Bus, BusWidth},
};

#[derive(Debug)]
pub(crate) struct Cpu {
    registers: Registers,
    pub(super) cpsr: CurrentProgramStatusRegister,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[expect(dead_code, reason = "Work in Progress")]
pub(crate) enum Primitive {
    Reg(Register, u32),
    Cpsr(CurrentProgramStatusRegister),
    Read {
        address: Address,
        width: BusWidth,
    },
    Write {
        address: Address,
        width: BusWidth,
        value: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Action {
    Primitive(Primitive),
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            cpsr: CurrentProgramStatusRegister::new(),
        }
    }
    pub(crate) fn reset(&mut self) {
        self.registers.reset();
        self.cpsr.reset();
    }
    pub(in crate::gba) fn step(&mut self, bus: &mut impl Bus) -> Time {
        let fetched_instruction = self.fetch(bus);
        let instruction = parse_arm(fetched_instruction);
        self.arm_exec(instruction, bus)
    }

    fn fetch(&mut self, bus: &mut impl Bus) -> Bitfield {
        let pc = self.registers[Register::R15];
        self.registers[Register::R15] = pc.wrapping_add(self.instruction_width());
        bus.read(Address(pc), BusWidth::B32)
    }

    pub(super) fn arm_exec(&mut self, instruction: ArmInstruction, bus: &mut impl Bus) -> Time {
        if self.check(instruction.condition) {
            let actions = compute_arm_actions(self, instruction.kind);
            for action in actions {
                self.apply(action, bus);
            }
        }
        Time(0)
    }
    fn apply(&mut self, action: Action, bus: &mut impl Bus) {
        match action {
            Action::Primitive(primitive) => match primitive {
                Primitive::Reg(register, value) => self.write(register, value),
                Primitive::Cpsr(cpsr) => self.cpsr = cpsr,
                Primitive::Read { address, width } => _ = bus.read(address, width),
                Primitive::Write {
                    address,
                    width,
                    value,
                } => bus.write(address, width, Bitfield::new(value)),
            },
        }
    }
    pub(super) fn read(&self, reg: Register) -> u32 {
        if let Register::R15 = reg {
            self.registers[reg].wrapping_add(self.instruction_width())
        } else {
            self.registers[reg]
        }
    }

    pub(super) fn write(&mut self, reg: Register, value: u32) {
        if let Register::R15 = reg {
            // TODO: Flush Precache pipeline when added
        }
        self.registers[reg] = value;
    }

    #[allow(clippy::unused_self, reason = "will be needed once thumb is added")]
    fn instruction_width(&self) -> u32 {
        //Pc incrementing necessarily follows the size of the instruction, so readahead on r15 reads is the same as pc incrementing on fetch.
        4
    }

    fn check(&self, condition: ArmCondition) -> bool {
        match condition {
            ArmCondition::Equal => self.cpsr.zero(),
            ArmCondition::NotEqual => !self.cpsr.zero(),
            ArmCondition::CarrySet => self.cpsr.carry(),
            ArmCondition::CarryCleared => !self.cpsr.carry(),
            ArmCondition::Minus => self.cpsr.negative(),
            ArmCondition::Plus => !self.cpsr.negative(),
            ArmCondition::SignedOverflow => self.cpsr.overflow(),
            ArmCondition::NoSignedOverflow => !self.cpsr.overflow(),
            ArmCondition::UnsignedHigher => self.cpsr.carry() && !self.cpsr.zero(),
            ArmCondition::UnsignedLowerOrSame => !self.cpsr.carry() || self.cpsr.zero(),
            ArmCondition::SignedGreaterEq => self.cpsr.negative() == self.cpsr.overflow(),
            ArmCondition::SignedLess => self.cpsr.negative() != self.cpsr.overflow(),
            ArmCondition::SignedGreater => {
                !self.cpsr.zero() && (self.cpsr.negative() == self.cpsr.overflow())
            }
            ArmCondition::SignedLessEq => {
                self.cpsr.zero() || (self.cpsr.negative() != self.cpsr.overflow())
            }
            ArmCondition::Always => true,
            ArmCondition::Never => false,
        }
    }
    pub(crate) fn operand_value(&self, operand: SourceOperand) -> (u32, Option<bool>) {
        match operand {
            SourceOperand::Immediate { value, shift } if shift != 0 => {
                let result = value.rotate_right(shift);
                let carry = (result & (1 << 31)) != 0;
                (result, Some(carry))
            }
            SourceOperand::Immediate { value, shift: _ } => (value, None),
            SourceOperand::Register(register) => (self.read(register), None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod source_operand {
        use super::*;
        fn immediate_test(value: u32, shift: u32) -> (u32, Option<bool>) {
            let op = SourceOperand::Immediate { value, shift };
            let cpu = Cpu::new();

            cpu.operand_value(op)
        }
        #[test]
        fn immediate_returns_none_on_no_shift() {
            let value = 0xFF;
            let shift = 0;
            let expected_result = (value, None);

            let actual_result = immediate_test(value, shift);

            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn immediate_returns_true_on_shift_with_31_set() {
            let value: u32 = 0xFF;
            let shift = 2;
            let expected_result = (value.rotate_right(shift), Some(true));

            let actual_result = immediate_test(value, shift);

            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn immediate_returns_false_on_shift_with_31_not_set() {
            let value: u32 = 0xFF;
            let shift = 10;
            let expected_result = (value.rotate_right(shift), Some(false));

            let actual_result = immediate_test(value, shift);

            assert_eq!(actual_result, expected_result);
        }
    }

    mod general_purpose {
        use super::*;
        mod arm {
            use super::*;
            use crate::gba::{
                instructions::{
                    ArmCondition, ArmInstruction, ArmKind, ArmOpCode, DataProcessing, SourceOperand,
                },
                memory::TestBus,
            };
            fn make_test_bus() -> TestBus {
                TestBus::new(vec![], vec![])
            }
            #[test]
            fn mov_sets_register_value_with_immediate() {
                let mut cpu = Cpu::new();
                let mut bus = make_test_bus();
                let register = Register::R0;
                let read_reg = Register::R1;
                let value = 10;
                let operand = SourceOperand::Immediate { value, shift: 0 };
                let original_read_reg_val = cpu.read(read_reg);
                let instruction = ArmInstruction {
                    condition: ArmCondition::Always,
                    kind: ArmKind::DataProcessing(DataProcessing {
                        op_code: ArmOpCode::Mov,
                        set_flag: false,
                        destination_reg: register,
                        read_reg,
                        source: operand,
                    }),
                };

                cpu.arm_exec(instruction, &mut bus);

                assert_eq!(cpu.read(register), value);
                assert_eq!(cpu.read(read_reg), original_read_reg_val);
            }
            #[test]
            fn mov_sets_register_with_pointed_register() {
                let mut cpu = Cpu::new();
                let mut bus = make_test_bus();
                let register = Register::R1;
                let read_reg = Register::R3;
                let write_reg = Register::R2;
                let value = 10;
                let operand = SourceOperand::Register(write_reg);
                cpu.write(write_reg, value);
                let original_read_reg_val = cpu.read(read_reg);
                let instruction = ArmInstruction {
                    condition: ArmCondition::Always,
                    kind: ArmKind::DataProcessing(DataProcessing {
                        op_code: ArmOpCode::Mov,
                        set_flag: false,
                        destination_reg: register,
                        read_reg,
                        source: operand,
                    }),
                };
                cpu.arm_exec(instruction, &mut bus);

                assert_eq!(cpu.read(register), value);
                assert_eq!(cpu.read(read_reg), original_read_reg_val);
            }

            mod check_conditions {
                use super::*;
                fn build_cpu(cpsr: CurrentProgramStatusRegister) -> Cpu {
                    Cpu {
                        registers: Registers::new(),
                        cpsr,
                    }
                }
                //Simple Cases
                #[test]
                fn equal() {
                    let condition = ArmCondition::Equal;
                    let true_case =
                        build_cpu(CurrentProgramStatusRegister::builder().zero().build());
                    let false_case = build_cpu(CurrentProgramStatusRegister::new());

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn not_equal() {
                    let condition = ArmCondition::NotEqual;
                    let true_case = build_cpu(CurrentProgramStatusRegister::new());
                    let false_case =
                        build_cpu(CurrentProgramStatusRegister::builder().zero().build());

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn carry_set() {
                    let condition = ArmCondition::CarrySet;
                    let true_case =
                        build_cpu(CurrentProgramStatusRegister::builder().carry().build());
                    let false_case = build_cpu(CurrentProgramStatusRegister::new());

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn carry_cleared() {
                    let condition = ArmCondition::CarryCleared;
                    let true_case = build_cpu(CurrentProgramStatusRegister::new());
                    let false_case =
                        build_cpu(CurrentProgramStatusRegister::builder().carry().build());

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn minus() {
                    let condition = ArmCondition::Minus;
                    let true_case =
                        build_cpu(CurrentProgramStatusRegister::builder().negative().build());
                    let false_case = build_cpu(CurrentProgramStatusRegister::new());

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn positive() {
                    let condition = ArmCondition::Plus;
                    let true_case = build_cpu(CurrentProgramStatusRegister::new());
                    let false_case =
                        build_cpu(CurrentProgramStatusRegister::builder().negative().build());

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn signed_overflow() {
                    let condition = ArmCondition::SignedOverflow;
                    let true_case =
                        build_cpu(CurrentProgramStatusRegister::builder().overflow().build());
                    let false_case = build_cpu(CurrentProgramStatusRegister::new());

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn no_signed_overflow() {
                    let condition = ArmCondition::NoSignedOverflow;
                    let true_case = build_cpu(CurrentProgramStatusRegister::new());
                    let false_case =
                        build_cpu(CurrentProgramStatusRegister::builder().overflow().build());

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn always() {
                    let condition = ArmCondition::Always;
                    let true_case =
                        build_cpu(CurrentProgramStatusRegister::builder().negative().build());

                    assert!(true_case.check(condition));
                }

                #[test]
                fn never() {
                    let condition = ArmCondition::Never;
                    let false_case =
                        build_cpu(CurrentProgramStatusRegister::builder().negative().build());

                    assert!(!false_case.check(condition));
                }

                //complex cases
                #[test]
                fn unsigned_higher() {
                    let condition = ArmCondition::UnsignedHigher;
                    let true_case =
                        build_cpu(CurrentProgramStatusRegister::builder().carry().build());
                    let false_case_1 = build_cpu(
                        CurrentProgramStatusRegister::builder()
                            .zero()
                            .carry()
                            .build(),
                    );
                    let false_case_2 = build_cpu(CurrentProgramStatusRegister::new());

                    assert!(true_case.check(condition));
                    assert!(!false_case_1.check(condition));
                    assert!(!false_case_2.check(condition));
                }

                #[test]
                fn unsigned_lower_or_same() {
                    let condition = ArmCondition::UnsignedLowerOrSame;
                    let true_case_1 = build_cpu(
                        CurrentProgramStatusRegister::builder()
                            .zero()
                            .carry()
                            .build(),
                    );
                    let true_case_2 = build_cpu(CurrentProgramStatusRegister::new());
                    let true_case_3 =
                        build_cpu(CurrentProgramStatusRegister::builder().zero().build());
                    let false_case =
                        build_cpu(CurrentProgramStatusRegister::builder().carry().build());

                    assert!(true_case_1.check(condition));
                    assert!(true_case_2.check(condition));
                    assert!(true_case_3.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn signed_greater_eq() {
                    let condition = ArmCondition::SignedGreaterEq;
                    let true_case_pos = build_cpu(CurrentProgramStatusRegister::new());
                    let true_case_neg = build_cpu(
                        CurrentProgramStatusRegister::builder()
                            .negative()
                            .overflow()
                            .build(),
                    );
                    let false_case_1 =
                        build_cpu(CurrentProgramStatusRegister::builder().overflow().build());
                    let false_case_2 =
                        build_cpu(CurrentProgramStatusRegister::builder().negative().build());

                    assert!(true_case_pos.check(condition));
                    assert!(true_case_neg.check(condition));
                    assert!(!false_case_1.check(condition));
                    assert!(!false_case_2.check(condition));
                }

                #[test]
                fn signed_less() {
                    let condition = ArmCondition::SignedLess;
                    let true_case_1 =
                        build_cpu(CurrentProgramStatusRegister::builder().overflow().build());
                    let true_case_2 =
                        build_cpu(CurrentProgramStatusRegister::builder().negative().build());
                    let false_case_pos = build_cpu(CurrentProgramStatusRegister::new());
                    let false_case_neg = build_cpu(
                        CurrentProgramStatusRegister::builder()
                            .negative()
                            .overflow()
                            .build(),
                    );

                    assert!(true_case_1.check(condition));
                    assert!(true_case_2.check(condition));
                    assert!(!false_case_pos.check(condition));
                    assert!(!false_case_neg.check(condition));
                }

                #[test]
                fn signed_greater() {
                    let condition = ArmCondition::SignedGreater;
                    let true_case_pos = build_cpu(CurrentProgramStatusRegister::new());
                    let true_case_neg = build_cpu(
                        CurrentProgramStatusRegister::builder()
                            .negative()
                            .overflow()
                            .build(),
                    );
                    let false_case_zero =
                        build_cpu(CurrentProgramStatusRegister::builder().zero().build());
                    let false_case_1 =
                        build_cpu(CurrentProgramStatusRegister::builder().overflow().build());
                    let false_case_2 =
                        build_cpu(CurrentProgramStatusRegister::builder().negative().build());

                    assert!(true_case_pos.check(condition));
                    assert!(true_case_neg.check(condition));
                    assert!(!false_case_zero.check(condition));
                    assert!(!false_case_1.check(condition));
                    assert!(!false_case_2.check(condition));
                }

                #[test]
                fn signed_less_eq() {
                    let condition = ArmCondition::SignedLessEq;
                    let true_case_zero =
                        build_cpu(CurrentProgramStatusRegister::builder().zero().build());
                    let true_case_1 =
                        build_cpu(CurrentProgramStatusRegister::builder().overflow().build());
                    let true_case_2 =
                        build_cpu(CurrentProgramStatusRegister::builder().negative().build());
                    let true_case_3 = build_cpu(
                        CurrentProgramStatusRegister::builder()
                            .negative()
                            .zero()
                            .build(),
                    );
                    let false_case_pos = build_cpu(CurrentProgramStatusRegister::new());
                    let false_case_neg = build_cpu(
                        CurrentProgramStatusRegister::builder()
                            .negative()
                            .overflow()
                            .build(),
                    );

                    assert!(true_case_zero.check(condition));
                    assert!(true_case_1.check(condition));
                    assert!(true_case_2.check(condition));
                    assert!(true_case_3.check(condition));
                    assert!(!false_case_pos.check(condition));
                    assert!(!false_case_neg.check(condition));
                }
            }
        }
    }
}

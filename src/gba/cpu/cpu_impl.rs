use std::ops::{Index, IndexMut};

use crate::gba::{
    bitmanip::Bitfield,
    clock::Time,
    cpu::{Register, flags::CurrentProgramStatusRegister, registers::Registers},
    instructions::{
        Instruction::{self, Arm},
        arm::ArmCondition,
        parse,
    },
    memory::{Bus, BusWidth},
};

#[derive(Debug)]
pub(crate) struct Cpu {
    registers: Registers,
    pub(super) cpsr: CurrentProgramStatusRegister,
}

impl Cpu {
    pub(crate) fn startup() -> Self {
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
        let instruction = parse(fetched_instruction);
        let Arm(command) = instruction;
        if self.check(command.condition) {
            self.run_arm(command);
        }
        Time(0)
    }

    fn fetch(&mut self, bus: &mut impl Bus) -> Bitfield {
        bus.read(self.registers.program_counter(), BusWidth::B32)
    }

    #[cfg_attr(not(test), expect(dead_code, reason = "Testing purposes"))]
    pub(in crate::gba::cpu) fn test_step(&mut self, instruction: Instruction, _bus: &mut impl Bus) {
        let Arm(arm_instr) = instruction;
        if self.check(arm_instr.condition) {
            self.run_arm(arm_instr);
        }
    }

    pub(super) fn read(&self, reg: Register) -> u32 {
        if let Register::R15 = reg {
            let program_count_step = 8;
            self.registers.index(reg).wrapping_add(program_count_step)
        } else {
            *self.registers.index(reg)
        }
    }
    pub(super) fn write(&mut self, reg: Register, value: u32) {
        if let Register::R15 = reg {
            // TODO: Flush Precache pipeline when added
        }
        *self.registers.index_mut(reg) = value;
    }

    fn check(&self, condition: ArmCondition) -> bool {
        match condition {
            ArmCondition::Equal => self.cpsr.get_zero_flag(),
            ArmCondition::NotEqual => !self.cpsr.get_zero_flag(),
            ArmCondition::CarrySet => self.cpsr.get_carry_flag(),
            ArmCondition::CarryCleared => !self.cpsr.get_carry_flag(),
            ArmCondition::Minus => self.cpsr.get_signed_flag(),
            ArmCondition::Plus => !self.cpsr.get_signed_flag(),
            ArmCondition::SignedOverflow => self.cpsr.get_overflow_flag(),
            ArmCondition::NoSignedOverflow => !self.cpsr.get_overflow_flag(),
            ArmCondition::UnsignedHigher => {
                self.cpsr.get_carry_flag() && !self.cpsr.get_zero_flag()
            }
            ArmCondition::UnsignedLowerOrSame => {
                !self.cpsr.get_carry_flag() || self.cpsr.get_zero_flag()
            }
            ArmCondition::SignedGreaterEq => {
                self.cpsr.get_signed_flag() == self.cpsr.get_overflow_flag()
            }
            ArmCondition::SignedLesser => {
                self.cpsr.get_signed_flag() != self.cpsr.get_overflow_flag()
            }
            ArmCondition::SignedGreater => {
                !self.cpsr.get_zero_flag()
                    && (self.cpsr.get_signed_flag() == self.cpsr.get_overflow_flag())
            }
            ArmCondition::SignedLesserEq => {
                self.cpsr.get_zero_flag()
                    || (self.cpsr.get_signed_flag() != self.cpsr.get_overflow_flag())
            }
            ArmCondition::Always => true,
            ArmCondition::Never => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod general_purpose {
        use super::*;
        mod arm {
            use super::*;
            use crate::gba::{
                instructions::arm::{ArmCommand, ArmCondition, ArmOpCode, SourceOperand},
                memory::TestBus,
            };
            fn make_test_bus() -> TestBus {
                TestBus::new(vec![], vec![])
            }
            #[test]
            fn mov_sets_register_value_with_immediate() {
                let mut cpu = Cpu::startup();
                let mut bus = make_test_bus();
                let register = Register::R0;
                let read_reg = Register::R1;
                let value = 10;
                let operand = SourceOperand::Immediate(value);
                let original_read_reg_val = cpu.read(read_reg);
                let instruction = Arm(ArmCommand {
                    condition: ArmCondition::Always,
                    op_code: ArmOpCode::Mov,
                    set_flag: false,
                    destination_reg: register,
                    read_reg,
                    source: operand,
                });

                cpu.test_step(instruction, &mut bus);

                assert_eq!(cpu.read(register), value);
                assert_eq!(cpu.read(read_reg), original_read_reg_val);
            }
            #[test]
            fn mov_sets_register_with_pointed_register() {
                let mut cpu = Cpu::startup();
                let mut bus = make_test_bus();
                let register = Register::R1;
                let read_reg = Register::R3;
                let write_reg = Register::R2;
                let value = 10;
                let operand = SourceOperand::Register(write_reg);
                cpu.write(write_reg, value);
                let original_read_reg_val = cpu.read(read_reg);
                let instruction = Arm(ArmCommand {
                    condition: ArmCondition::Always,
                    op_code: ArmOpCode::Mov,
                    set_flag: false,
                    destination_reg: register,
                    read_reg,
                    source: operand,
                });
                cpu.test_step(instruction, &mut bus);

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
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, true, false, false,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn not_equal() {
                    let condition = ArmCondition::NotEqual;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, true, false, false,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn carry_set() {
                    let condition = ArmCondition::CarrySet;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, true, false,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn carry_cleared() {
                    let condition = ArmCondition::CarryCleared;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, true, false,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn minus() {
                    let condition = ArmCondition::Minus;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, false,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn positive() {
                    let condition = ArmCondition::Plus;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, false,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn signed_overflow() {
                    let condition = ArmCondition::SignedOverflow;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, true,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn no_signed_overflow() {
                    let condition = ArmCondition::NoSignedOverflow;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, true,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn always() {
                    let condition = ArmCondition::Always;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, false,
                    ));

                    assert!(true_case.check(condition));
                }

                #[test]
                fn never() {
                    let condition = ArmCondition::Never;
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, false,
                    ));

                    assert!(!false_case.check(condition));
                }

                //complex cases
                #[test]
                fn unsigned_higher() {
                    let condition = ArmCondition::UnsignedHigher;
                    let true_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, true, false,
                    ));
                    let false_case_1 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, true, true, false,
                    ));
                    let false_case_2 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));

                    assert!(true_case.check(condition));
                    assert!(!false_case_1.check(condition));
                    assert!(!false_case_2.check(condition));
                }

                #[test]
                fn unsigned_lower_or_same() {
                    let condition = ArmCondition::UnsignedLowerOrSame;
                    let true_case_1 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, true, true, false,
                    ));
                    let true_case_2 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let true_case_3 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, true, false, false,
                    ));
                    let false_case = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, true, false,
                    ));

                    assert!(true_case_1.check(condition));
                    assert!(true_case_2.check(condition));
                    assert!(true_case_3.check(condition));
                    assert!(!false_case.check(condition));
                }

                #[test]
                fn signed_greater_eq() {
                    let condition = ArmCondition::SignedGreaterEq;
                    let true_case_pos = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let true_case_neg = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, true,
                    ));
                    let false_case_1 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, true,
                    ));
                    let false_case_2 = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, false,
                    ));

                    assert!(true_case_pos.check(condition));
                    assert!(true_case_neg.check(condition));
                    assert!(!false_case_1.check(condition));
                    assert!(!false_case_2.check(condition));
                }

                #[test]
                fn signed_lesser() {
                    let condition = ArmCondition::SignedLesser;
                    let true_case_1 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, true,
                    ));
                    let true_case_2 = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, false,
                    ));
                    let false_case_pos = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let false_case_neg = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, true,
                    ));

                    assert!(true_case_1.check(condition));
                    assert!(true_case_2.check(condition));
                    assert!(!false_case_pos.check(condition));
                    assert!(!false_case_neg.check(condition));
                }

                #[test]
                fn signed_greater() {
                    let condition = ArmCondition::SignedGreater;
                    let true_case_pos = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let true_case_neg = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, true,
                    ));
                    let false_case_zero = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, true, false, false,
                    ));
                    let false_case_1 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, true,
                    ));
                    let false_case_2 = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, false,
                    ));

                    assert!(true_case_pos.check(condition));
                    assert!(true_case_neg.check(condition));
                    assert!(!false_case_zero.check(condition));
                    assert!(!false_case_1.check(condition));
                    assert!(!false_case_2.check(condition));
                }

                #[test]
                fn signed_lesser_eq() {
                    let condition = ArmCondition::SignedLesserEq;
                    let true_case_zero = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, true, false, false,
                    ));
                    let true_case_1 = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, true,
                    ));
                    let true_case_2 = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, false,
                    ));
                    let true_case_3 = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, true, false, false,
                    ));
                    let false_case_pos = build_cpu(CurrentProgramStatusRegister::with_values(
                        false, false, false, false,
                    ));
                    let false_case_neg = build_cpu(CurrentProgramStatusRegister::with_values(
                        true, false, false, true,
                    ));

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

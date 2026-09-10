use std::ops::{Index, IndexMut};

use crate::{
    cpu::{flags::CurrentProgramStatusRegister, registers::Registers},
    instructions::Instruction::{self, Arm},
};
pub(crate) use registers::Register;

mod arm_exec;
mod flags;
mod registers;

#[derive(Debug)]
pub(super) struct Cpu {
    registers: Registers,
    cpsr: CurrentProgramStatusRegister,
}

impl Cpu {
    //TODO: add static function for initial values,
    // and reset function to quickly restart
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            cpsr: CurrentProgramStatusRegister::new(),
        }
    }
    pub fn step(&mut self, instruction: Instruction) {
        let Arm(arm_instr) = instruction;
        self.run_arm(arm_instr);
    }
    fn read(&self, reg: Register) -> u32 {
        if let Register::R15 = reg {
            self.registers.index(reg).overflowing_add(8).0
        } else {
            *self.registers.index(reg)
        }
    }
    fn write(&mut self, reg: Register, value: u32) {
        if let Register::R15 = reg {
            // TODO: Flush Precache pipeline when added
        } else {
            *self.registers.index_mut(reg) = value;
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
            use crate::instructions::{
                SourceOperand,
                arm::{ArmCommand, ArmCondition, ArmOpCode},
            };
            #[test]
            fn mov_sets_register_value_with_immediate() {
                let mut cpu = Cpu::new();
                let register = Register::R0;
                let read_reg = Register::R1;
                let value = 10;
                let operand = SourceOperand::Immediate(value);
                let original_read_reg_val = cpu.read(read_reg);
                let instruction = Arm(ArmCommand {
                    condition: ArmCondition::Temp,
                    op_code: ArmOpCode::Mov,
                    set_flag: false,
                    destination_reg: register,
                    read_reg,
                    source: operand,
                });

                cpu.step(instruction);

                assert_eq!(cpu.read(register), value);
                assert_eq!(cpu.read(read_reg), original_read_reg_val);
            }
            #[test]
            fn mov_sets_register_with_pointed_register() {
                let mut cpu = Cpu::new();
                let register = Register::R1;
                let read_reg = Register::R3;
                let write_reg = Register::R2;
                let value = 10;
                let operand = SourceOperand::Register(write_reg);
                cpu.write(write_reg, value);
                let original_read_reg_val = cpu.read(read_reg);
                let instruction = Arm(ArmCommand {
                    condition: ArmCondition::Temp,
                    op_code: ArmOpCode::Mov,
                    set_flag: false,
                    destination_reg: register,
                    read_reg,
                    source: operand,
                });
                cpu.step(instruction);

                assert_eq!(cpu.read(register), value);
                assert_eq!(cpu.read(read_reg), original_read_reg_val);
            }
        }
    }
}

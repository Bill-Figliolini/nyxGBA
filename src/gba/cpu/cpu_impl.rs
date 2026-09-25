use std::ops::{Index, IndexMut};

use crate::gba::{
    cpu::{Register, flags::CurrentProgramStatusRegister, registers::Registers},
    instructions::Instruction::{self, Arm},
};

#[derive(Debug)]
pub(crate) struct Cpu {
    registers: Registers,
    pub(super) cpsr: CurrentProgramStatusRegister,
}

pub(crate) fn startup() -> Cpu {
    Cpu::new()
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
    pub(in crate::gba) fn step(&mut self, instruction: Instruction) {
        let Arm(arm_instr) = instruction;
        self.run_arm(arm_instr);
    }
    pub(super) fn read(&self, reg: Register) -> u32 {
        if let Register::R15 = reg {
            self.registers.index(reg).overflowing_add(8).0
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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod general_purpose {
        use super::*;
        mod arm {
            use super::*;
            use crate::gba::instructions::arm::{
                ArmCommand, ArmCondition, ArmOpCode, SourceOperand,
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
                    condition: ArmCondition::Always,
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
                    condition: ArmCondition::Always,
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

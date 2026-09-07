use std::ops::{Index, IndexMut};

use crate::{
    cpu::registers::Registers,
    instructions::Instruction::{self, ArmAlu},
};
pub(crate) use registers::Register;

mod arm_exec;
mod registers;

#[derive(Debug)]
pub(super) struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn init() -> Self {
        Cpu {
            registers: Registers::init(),
        }
    }
    pub fn run(&mut self, instruction: Instruction) {
        let ArmAlu(arm_instr) = instruction;
        self.run_arm(arm_instr);
    }
    fn get(&self, reg: Register) -> &u32 {
        self.registers.index(reg)
    }
    fn set(&mut self, reg: Register, value: u32) {
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
            use crate::instructions::{self, arm_alu::ArmOpCode};
            use instructions::{MovArgs, SecondOperand};
            #[test]
            fn mov_sets_register_value_with_immediate() {
                let mut cpu = Cpu::init();
                let register = Register::R0;
                let value = 10;
                let operand = SecondOperand::Immediate(value);
                let instruction_args = MovArgs {
                    destination: register,
                    source: operand,
                };
                let instruction = ArmAlu(ArmOpCode::Mov(instruction_args));

                cpu.run(instruction);

                assert_eq!(*cpu.get(register), value);
            }
            #[test]
            fn mov_sets_register_with_pointed_register() {
                let mut cpu = Cpu::init();
                let source_register = Register::R2;
                let destination_register = Register::R1;
                let value = 10;
                cpu.set(source_register, value);
                let operand = SecondOperand::Register(source_register);
                let instruction_args = MovArgs {
                    destination: destination_register,
                    source: operand,
                };
                let instruction = ArmAlu(ArmOpCode::Mov(instruction_args));

                cpu.run(instruction);

                assert_eq!(*cpu.get(destination_register), value);
            }
        }
    }
}

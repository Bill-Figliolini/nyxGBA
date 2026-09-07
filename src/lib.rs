use crate::{
    cpu::{Cpu, Register},
    instructions::{Instruction, MovArgs, SecondOperand, arm_alu::ArmOpCode},
};

mod cpu;
mod instructions;

pub fn nyx_main() {
    let mut cpu = Cpu::init();
    let value = MovArgs {
        destination: Register::R0,
        source: SecondOperand::Immediate(10),
    };
    let command = Instruction::ArmAlu(ArmOpCode::Mov(value));
    cpu.run(command);
}

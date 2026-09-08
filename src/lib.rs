use crate::{
    cpu::{Cpu, Register},
    instructions::{
        Instruction::Arm,
        SourceOperand,
        arm::{ArmCommand, ArmCondition, ArmOpCode},
    },
};

mod cpu;
mod instructions;
mod memory;

pub fn nyx_main() {
    let mut cpu = Cpu::new();
    let register = Register::R0;
    let read_reg = Register::R1;
    let value = 10;
    let operand = SourceOperand::Immediate(value);
    let instruction = Arm(ArmCommand {
        condition: ArmCondition::Temp,
        op_code: ArmOpCode::Mov,
        set_flag: false,
        destination_reg: register,
        read_reg,
        source: operand,
    });
    cpu.run(instruction);
}

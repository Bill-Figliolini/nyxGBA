//! # parsing.rs
//! parses generated bitfields into their relevant instructions

use crate::gba::{
    bitmanip::Bitfield,
    cpu::Register,
    instructions::{
        Instruction::{self},
        arm::{ArmCommand, ArmCondition, ArmOpCode, SourceOperand},
    },
};

pub(crate) fn parse(input: Bitfield) -> Instruction {
    let condition = ArmCondition::new(input.get_range(28, 31));
    let register = Register::R0;
    let read_reg = Register::R1;
    let value = 10;
    let operand = SourceOperand::Immediate(value);
    let instruction = ArmCommand {
        condition,
        op_code: ArmOpCode::Mov,
        set_flag: false,
        destination_reg: register,
        read_reg,
        source: operand,
    };
    Instruction::Arm(instruction)
}

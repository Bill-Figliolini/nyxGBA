//! # parsing.rs
//! parses generated bitfields into their relevant instructions

use crate::{
    bitmanip::Bitfield,
    cpu::Register,
    instructions::{
        Instruction::{self},
        SourceOperand,
        arm::{ArmCommand, ArmCondition, ArmOpCode},
    },
};

pub(crate) fn parse(_input: Bitfield) -> Instruction {
    let register = Register::R0;
    let read_reg = Register::R1;
    let value = 10;
    let operand = SourceOperand::Immediate(value);
    let instruction = ArmCommand {
        condition: ArmCondition::Temp,
        op_code: ArmOpCode::Mov,
        set_flag: false,
        destination_reg: register,
        read_reg,
        source: operand,
    };
    Instruction::Arm(instruction)
}

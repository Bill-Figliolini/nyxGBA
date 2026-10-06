//! # parsing.rs
//! parses generated bitfields into their relevant instructions

use crate::gba::{
    bitmanip::Bitfield,
    cpu::Register,
    instructions::arm::{ArmCommand, ArmCondition, ArmOpCode, SourceOperand},
};

pub(crate) fn parse_arm(input: Bitfield) -> ArmCommand {
    let condition = ArmCondition::new(input.get_range(28, 4));
    let register = Register::R0;
    let read_reg = Register::R1;
    let value = 10;
    let operand = SourceOperand::Immediate(value);
    ArmCommand {
        condition,
        op_code: ArmOpCode::Mov,
        set_flag: false,
        destination_reg: register,
        read_reg,
        source: operand,
    }
}

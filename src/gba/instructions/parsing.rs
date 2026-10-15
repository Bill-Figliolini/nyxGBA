//! # parsing.rs
//! parses generated bitfields into their relevant instructions

use crate::gba::{
    bitmanip::Bitfield,
    cpu::Register,
    instructions::arm::{
        ArmCondition, ArmInstruction, ArmKind, ArmOpCode, DataProcessing, SourceOperand,
    },
};

pub(crate) fn parse_arm(input: Bitfield) -> ArmInstruction {
    let condition = ArmCondition::new(input.bits(28, 4));
    let destination_reg = Register::R0;
    let read_reg = Register::R1;
    let set_flag = input.bit(20);
    let op_code_bits = input.bits(21, 4);
    let op_code = determine_opcode(op_code_bits);
    let value = 10;
    let source = SourceOperand::Immediate(value);
    ArmInstruction {
        condition,
        kind: ArmKind::DataProcessing(DataProcessing {
            op_code,
            set_flag,
            destination_reg,
            read_reg,
            source,
        }),
    }
}
fn determine_opcode(_code: u32) -> ArmOpCode {
    ArmOpCode::Mov
}

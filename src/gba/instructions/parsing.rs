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
    let kind = if is_data_processing(input) {
        parse_data_processing(input)
    } else {
        ArmKind::Unimplemented(input)
    };
    ArmInstruction { condition, kind }
}

fn determine_opcode(instr: Bitfield) -> ArmOpCode {
    let code = instr.bits(21, 4);
    match code {
        0xD => ArmOpCode::Mov,
        0x4 => ArmOpCode::Add,
        _ => ArmOpCode::Unimplemented(instr),
    }
}
fn is_data_processing(input: Bitfield) -> bool {
    let bits_26_27 = input.bits(26, 2);

    //TODO: Add holes, when relevant
    bits_26_27 == 0b00
}

fn parse_data_processing(input: Bitfield) -> ArmKind {
    let op_code = determine_opcode(input);
    let set_flag = input.bit(20);
    let destination_reg = Register::from_bits(input, 12);
    let read_reg = Register::from_bits(input, 16);
    let source = SourceOperand::from_bits(input);
    ArmKind::DataProcessing(DataProcessing {
        op_code,
        set_flag,
        destination_reg,
        read_reg,
        source,
    })
}

#[cfg(test)]
mod tests {
    // Responsible only for the kind field of an instruction
    // Conditions are in arm.rs
    use super::*;

    mod data_processing {
        use crate::gba::instructions::arm::ShiftType;

        use super::*;
        #[test]
        fn mov_instruction() {
            // cond=AL | 00 | I=1 | MOV | S=1 | Rn=R10 | Rd=R0 | rot=0 | imm=20
            let instruction = Bitfield::new(0b1110_0011_1011_1010_0000_0000_0001_0100);
            let op_code = ArmOpCode::Mov;
            let read_reg = Register::R10;
            let destination_reg = Register::R0;
            let set_flag = true;
            let source = SourceOperand::Immediate {
                value: 20,
                shift: 0,
            };
            let expected_output = ArmKind::DataProcessing(DataProcessing {
                op_code,
                set_flag,
                destination_reg,
                read_reg,
                source,
            });

            let actual_output = parse_data_processing(instruction);

            assert_eq!(actual_output, expected_output);
        }
        #[test]
        fn add_instruction() {
            // cond=AL | 00 | I=0 | ADD | S=1 | Rn=R7 | Rd=R8 | shift=0 | LSL | 0 | Rm=R6
            let instruction = Bitfield::new(0b1110_0000_1001_0111_1000_0000_0000_0110);
            let op_code = ArmOpCode::Add;
            let read_reg = Register::R7;
            let destination_reg = Register::R8;
            let set_flag = true;
            let source = SourceOperand::Register {
                register: Register::R6,
                shift_type: ShiftType::None,
            };
            let expected_output = ArmKind::DataProcessing(DataProcessing {
                op_code,
                set_flag,
                destination_reg,
                read_reg,
                source,
            });

            let actual_output = parse_data_processing(instruction);

            assert_eq!(actual_output, expected_output);
        }
    }
}

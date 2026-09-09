use crate::{cpu::Register, instructions::arm::ArmCommand};

pub(crate) mod arm;

#[derive(Debug)]
pub(crate) enum Instruction {
    Arm(ArmCommand),
}

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "To be used later"))]
pub(crate) enum SourceOperand {
    Immediate(u32),
    Register(Register),
}

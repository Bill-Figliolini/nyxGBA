use crate::instructions::arm::ArmCommand;
pub(crate) mod parsing;
pub(crate) mod arm;

pub(crate) use parsing::parse;

#[derive(Debug)]
pub(crate) enum Instruction {
    Arm(ArmCommand),
}

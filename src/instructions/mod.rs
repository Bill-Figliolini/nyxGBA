use crate::instructions::arm::ArmCommand;

pub(crate) mod arm;

#[derive(Debug)]
pub(crate) enum Instruction {
    Arm(ArmCommand),
}

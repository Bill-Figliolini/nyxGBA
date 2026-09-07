use crate::instructions::MovArgs;

#[derive(Debug)]
pub(crate) enum ArmOpCode {
    Mov(MovArgs),
}

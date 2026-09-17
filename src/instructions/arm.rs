use crate::{cpu::Register, instructions::SourceOperand};

#[derive(Debug)]
#[expect(dead_code, reason = "To be used later")]
pub(crate) struct ArmCommand {
    pub condition: ArmCondition,
    pub op_code: ArmOpCode,
    pub set_flag: bool,
    pub destination_reg: Register,
    pub read_reg: Register,
    pub source: SourceOperand,
}
#[derive(Debug)]
#[expect(dead_code, reason = "To be used Later")]
pub(crate) enum ArmCondition {
    Equal,
    NotEqual,
    CarrySet,
    CarryCleared,
    Minus,
    Plus,
    SignedOverflow,
    NoSignedOverflow,
    UnsignedHigher,
    UnsignedLowerOrSame,
    SignedGreaterEq,
    SignedLesser,
    SignedGreater,
    SignedLesserEq,
    Always,
    Never,
}

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "To be used later"))]
pub(crate) enum ArmOpCode {
    Mov,
    Add,
}

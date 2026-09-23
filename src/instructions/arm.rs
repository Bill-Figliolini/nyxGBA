use crate::cpu::Register;

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
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
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

impl ArmCondition {
    pub(crate) fn new(input: u32) -> Self {
        debug_assert!(input < 0x10);
        // SAFETY:
        // input must be less than 0x10
        //can now be infallibly converted to Armcondition
        #[allow(clippy::as_conversions, reason = "input is restricted to less than 0x10, which fits in a u8")]
        #[allow(clippy::cast_possible_truncation, reason = "input is restricted to less than 0x10, which fits in a u8")]
        unsafe { std::mem::transmute(input as u8)}

    }
}

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "To be used later"))]
pub(crate) enum ArmOpCode {
    Mov,
    Add,
}

#[derive(Debug)]
#[cfg_attr(not(test), expect(dead_code, reason = "To be used later"))]
pub(crate) enum SourceOperand {
    Immediate(u32),
    Register(Register),
}

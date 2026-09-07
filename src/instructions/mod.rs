use crate::cpu::Register;

pub(crate) mod arm_alu;

#[derive(Debug)]
pub(crate) enum Instruction {
    ArmAlu(arm_alu::ArmOpCode),
}

#[derive(Debug)]
pub(crate) struct MovArgs {
    pub destination: Register,
    pub source: SecondOperand,
}

//TODO: Add Register Access
#[derive(Debug)]
#[allow(dead_code, reason = "will be used later")]
pub(crate) enum SecondOperand {
    Immediate(i32),
    Register(Register),
}

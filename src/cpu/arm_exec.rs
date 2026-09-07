use crate::{
    Cpu,
    instructions::{MovArgs, SecondOperand, arm_alu::ArmOpCode},
};

impl Cpu {
    pub(super) fn run_arm(&mut self, op: ArmOpCode) {
        match op {
            ArmOpCode::Mov(mov_args) => self.arm_mov(&mov_args),
        }
    }
    fn arm_mov(&mut self, mov_args: &MovArgs) {
        let value = match mov_args.source {
            SecondOperand::Immediate(val) => val,
            SecondOperand::Register(register) => *self.get(register),
        };
        self.set(mov_args.destination, value);
    }
}

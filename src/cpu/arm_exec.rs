use crate::{
    Cpu,
    instructions::{
        SourceOperand,
        arm::{ArmCommand, ArmOpCode},
    },
};

impl Cpu {
    pub(super) fn run_arm(&mut self, arm: ArmCommand) {
        match arm.op_code {
            ArmOpCode::Mov => self.arm_mov(arm),
            ArmOpCode::Add => self.arm_add(arm),
        }
    }
    fn arm_mov(&mut self, command: ArmCommand) {
        let ArmCommand {
            condition: _,
            op_code: _,
            set_flag: _,
            destination_reg: destination,
            read_reg: _,
            source,
        } = command;
        let value = match source {
            SourceOperand::Immediate(val) => val,
            SourceOperand::Register(register) => self.get(register),
        };
        self.set(destination, value);
    }
    fn arm_add(&mut self, command: ArmCommand) {
        let ArmCommand {
            condition: _,
            op_code: _,
            set_flag,
            destination_reg: destination,
            read_reg: read,
            source,
        } = command;
        let r_value = match source {
            SourceOperand::Immediate(val) => val,
            SourceOperand::Register(register) => self.get(register),
        };
        let (result, _overflow) = self.get(read).overflowing_add(r_value);
        if set_flag {
            self.cpsr.set_zero_flag(result == 0);
            self.cpsr
                .set_signed_flag(result.cast_signed().is_negative());
        }

        self.set(destination, result);
    }
}

//TODO:
// Add tests for add. 5 cases:
// set_flag on, standard
// set_flag off, standard, existing flags
// set_flag on, overflow
// set_flag on, 0 result

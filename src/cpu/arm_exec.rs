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
            SourceOperand::Register(register) => self.read(register),
        };
        self.write(destination, value);
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
            SourceOperand::Register(register) => self.read(register),
        };
        let (result, carry) = self.read(read).overflowing_add(r_value);
        if set_flag {
            self.cpsr.set_zero_flag(result == 0);
            self.cpsr
                .set_signed_flag(result.cast_signed().is_negative());
            self.cpsr.set_carry_flag(carry);
            self.cpsr
                .set_overflow_flag(result > i32::MAX.cast_unsigned());
        }

        self.write(destination, result);
    }
}

//TODO:
// Add tests for add. 5 cases:
// set_flag on, standard
// set_flag off, standard, existing flags
// set_flag on,
// set_flag on, 0 result
#[cfg(test)]
mod tests {
    use super::*;
    mod add {
        use crate::{cpu::Register, instructions::Instruction};

        use super::*;

        #[test]
        fn carry_flag_set_on_overflow() {
            let mut cpu = Cpu::new();
            let dest = Register::R1;
            let reg = Register::R0;
            cpu.write(reg, u32::MAX);
            let instr = Instruction::Arm(ArmCommand {
                condition: crate::instructions::arm::ArmCondition::Temp,
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg: dest,
                read_reg: reg,
                source: SourceOperand::Immediate(1),
            });

            cpu.step(instr);

            assert_eq!(cpu.read(dest), 0);
            assert!(cpu.cpsr.get_carry_flag());
        }

        #[test]
        fn set_flag_on_zero() {
            let mut cpu = Cpu::new();
            let dest = Register::R1;
            let reg = Register::R0;
            cpu.write(reg, 0);
            let instr = Instruction::Arm(ArmCommand {
                condition: crate::instructions::arm::ArmCondition::Temp,
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg: dest,
                read_reg: reg,
                source: SourceOperand::Immediate(0),
            });

            cpu.step(instr);

            assert_eq!(cpu.read(dest), 0);
            assert!(cpu.cpsr.get_zero_flag());
        }
        #[test]
        fn set_overflow_flag_on_signed_overflow() {
            let mut cpu = Cpu::new();
            let dest = Register::R1;
            let reg = Register::R0;
            cpu.write(reg, i32::MAX.cast_unsigned());
            let instr = Instruction::Arm(ArmCommand {
                condition: crate::instructions::arm::ArmCondition::Temp,
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg: dest,
                read_reg: reg,
                source: SourceOperand::Immediate(1),
            });

            cpu.step(instr);

            assert!(cpu.cpsr.get_overflow_flag());
        }
    }
}

use crate::gba::{
    cpu::cpu_impl::Cpu,
    instructions::arm::{ArmInstruction, ArmOpCode, SourceOperand},
    memory::Bus,
};

impl Cpu {
    pub(super) fn run_arm(&mut self, arm: ArmInstruction, _bus: &mut impl Bus) {
        match arm.op_code {
            ArmOpCode::Mov => self.arm_mov(arm),
            ArmOpCode::Add => self.arm_add(arm),
        }
    }
    fn arm_mov(&mut self, command: ArmInstruction) {
        let ArmInstruction {
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
    fn arm_add(&mut self, command: ArmInstruction) {
        let ArmInstruction {
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
        let l_value = self.read(read);
        let (result, carry) = l_value.overflowing_add(r_value);
        let overflow = l_value
            .cast_signed()
            .overflowing_add(r_value.cast_signed())
            .1;
        if set_flag {
            self.cpsr.set_zero(result == 0);
            self.cpsr.set_negative(result.cast_signed().is_negative());
            self.cpsr.set_carry(carry);
            self.cpsr.set_overflow(overflow);
        }

        self.write(destination, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod add {
        use crate::gba::{cpu::Register, instructions, memory::TestBus};

        use super::*;

        fn make_test_bus() -> TestBus {
            TestBus::new(vec![], vec![])
        }

        #[test]
        fn carry_flag_set_on_overflow() {
            let mut cpu = Cpu::new();
            let mut bus = make_test_bus();
            let dest = Register::R1;
            let reg = Register::R0;
            cpu.write(reg, u32::MAX);
            let instr = ArmInstruction {
                condition: instructions::arm::ArmCondition::Always,
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg: dest,
                read_reg: reg,
                source: SourceOperand::Immediate(1),
            };

            cpu.arm_exec(instr, &mut bus);

            assert_eq!(cpu.read(dest), 0);
            assert!(cpu.cpsr.carry());
        }

        #[test]
        fn set_flag_on_zero() {
            let mut cpu = Cpu::new();
            let mut bus = make_test_bus();
            let dest = Register::R1;
            let reg = Register::R0;
            cpu.write(reg, 0);
            let instr = ArmInstruction {
                condition: instructions::arm::ArmCondition::Always,
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg: dest,
                read_reg: reg,
                source: SourceOperand::Immediate(0),
            };

            cpu.arm_exec(instr, &mut bus);

            assert_eq!(cpu.read(dest), 0);
            assert!(cpu.cpsr.zero());
        }
        #[test]
        fn set_overflow_flag_on_signed_overflow() {
            let mut cpu = Cpu::new();
            let mut bus = make_test_bus();
            let dest = Register::R1;
            let reg = Register::R0;
            cpu.write(reg, i32::MAX.cast_unsigned());
            let instr = ArmInstruction {
                condition: instructions::arm::ArmCondition::Always,
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg: dest,
                read_reg: reg,
                source: SourceOperand::Immediate(1),
            };

            cpu.arm_exec(instr, &mut bus);

            assert!(cpu.cpsr.overflow());
        }
        #[test]
        fn do_not_set_overflow_flag_on_negative_addition() {
            let mut cpu = Cpu::new();
            let mut bus = make_test_bus();
            let dest = Register::R1;
            let reg = Register::R0;
            let val: i32 = -1;
            cpu.write(reg, val.cast_unsigned());
            let instr = ArmInstruction {
                condition: instructions::arm::ArmCondition::Always,
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg: dest,
                read_reg: reg,
                source: SourceOperand::Immediate(val.cast_unsigned()),
            };

            cpu.arm_exec(instr, &mut bus);

            assert!(!cpu.cpsr.overflow());
        }
        #[test]
        fn set_overflow_flag_on_overflow() {
            let mut cpu = Cpu::new();
            let mut bus = make_test_bus();
            let dest = Register::R1;
            let reg = Register::R0;
            cpu.write(reg, 0x8000_0000);
            let instr = ArmInstruction {
                condition: instructions::arm::ArmCondition::Always,
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg: dest,
                read_reg: reg,
                source: SourceOperand::Immediate(0x8000_0000),
            };

            cpu.arm_exec(instr, &mut bus);

            assert!(cpu.cpsr.overflow());
        }
    }
}

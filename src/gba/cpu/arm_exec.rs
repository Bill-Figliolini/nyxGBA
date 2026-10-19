use crate::{
    gba::{
        cpu::cpu_impl::Cpu,
        instructions::{ArmKind, ArmOpCode, DataProcessing},
        memory::Bus,
    },
    log_error,
};

impl Cpu {
    pub(super) fn run_arm(&mut self, arm: ArmKind, _bus: &mut impl Bus) {
        match arm {
            ArmKind::DataProcessing(data_processing) => {
                self.run_data_processing(data_processing);
            }
            ArmKind::Unimplemented(instr) => {
                log_error(&format!("Unknown instruction found: {instr}"));
            }
        }
    }
    fn run_data_processing(&mut self, instr: DataProcessing) {
        match instr.op_code {
            ArmOpCode::Mov => self.arm_mov(instr),
            ArmOpCode::Add => self.arm_add(instr),
            ArmOpCode::Unimplemented(code) => {
                log_error(&format!("Unknown instruction found: {code}"));
            }
        }
    }
    fn arm_mov(&mut self, command: DataProcessing) {
        let DataProcessing {
            set_flag,
            destination_reg: destination,
            source,
            ..
        } = command;
        let (value, carry) = self.operand_value(source);
        if set_flag {
            self.cpsr.set_nz(value);
            self.cpsr.set_carry(carry);
        }
        self.write(destination, value);
    }
    fn arm_add(&mut self, command: DataProcessing) {
        let DataProcessing {
            set_flag,
            destination_reg: destination,
            read_reg: read,
            source,
            ..
        } = command;
        let (r_value, _) = self.operand_value(source);
        let l_value = self.read(read);
        let (result, carry) = l_value.overflowing_add(r_value);
        let overflow = l_value
            .cast_signed()
            .overflowing_add(r_value.cast_signed())
            .1;
        if set_flag {
            self.cpsr.set_nz(result);
            self.cpsr.set_carry(carry);
            self.cpsr.set_overflow(overflow);
        }

        self.write(destination, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::{
        cpu::Register,
        instructions::{ArmCondition, ArmInstruction, SourceOperand},
        memory::TestBus,
    };
    mod mov {
        use super::*;

        fn make_test_bus() -> TestBus {
            TestBus::new(vec![], vec![])
        }

        fn mov_instr(set_flag: bool, value: u32) -> ArmInstruction {
            ArmInstruction {
                condition: ArmCondition::Always,
                kind: ArmKind::DataProcessing(DataProcessing {
                    op_code: ArmOpCode::Mov,
                    set_flag,
                    destination_reg: Register::R1,
                    read_reg: Register::R0,
                    source: SourceOperand::Immediate { value, shift: 0 },
                }),
            }
        }

        #[test]
        fn set_zero_flag_on_zero() {
            let mut cpu = Cpu::new();
            let mut bus = make_test_bus();

            cpu.arm_exec(mov_instr(true, 0), &mut bus);

            assert_eq!(cpu.read(Register::R1), 0);
            assert!(cpu.cpsr.zero());
            assert!(!cpu.cpsr.negative());
        }
        #[test]
        fn set_negative_flag_on_high_bit() {
            let mut cpu = Cpu::new();
            let mut bus = make_test_bus();

            cpu.arm_exec(mov_instr(true, 0x8000_0000), &mut bus);

            assert!(cpu.cpsr.negative());
            assert!(!cpu.cpsr.zero());
        }
        #[test]
        fn do_not_set_flags_without_set_flag() {
            let mut cpu = Cpu::new();
            let mut bus = make_test_bus();

            cpu.arm_exec(mov_instr(false, 0), &mut bus);

            assert!(!cpu.cpsr.zero());
            assert!(!cpu.cpsr.negative());
        }
    }
    mod add {

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
                condition: ArmCondition::Always,
                kind: ArmKind::DataProcessing(DataProcessing {
                    op_code: ArmOpCode::Add,
                    set_flag: true,
                    destination_reg: dest,
                    read_reg: reg,
                    source: SourceOperand::Immediate { value: 1, shift: 0 },
                }),
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
                condition: ArmCondition::Always,
                kind: ArmKind::DataProcessing(DataProcessing {
                    op_code: ArmOpCode::Add,
                    set_flag: true,
                    destination_reg: dest,
                    read_reg: reg,
                    source: SourceOperand::Immediate { value: 0, shift: 0 },
                }),
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
                condition: ArmCondition::Always,
                kind: ArmKind::DataProcessing(DataProcessing {
                    op_code: ArmOpCode::Add,
                    set_flag: true,
                    destination_reg: dest,
                    read_reg: reg,
                    source: SourceOperand::Immediate { value: 1, shift: 0 },
                }),
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
                condition: ArmCondition::Always,
                kind: ArmKind::DataProcessing(DataProcessing {
                    op_code: ArmOpCode::Add,
                    set_flag: true,
                    destination_reg: dest,
                    read_reg: reg,
                    source: SourceOperand::Immediate {
                        value: val.cast_unsigned(),
                        shift: 0,
                    },
                }),
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
                condition: ArmCondition::Always,
                kind: ArmKind::DataProcessing(DataProcessing {
                    op_code: ArmOpCode::Add,
                    set_flag: true,
                    destination_reg: dest,
                    read_reg: reg,
                    source: SourceOperand::Immediate {
                        value: 0x8000_0000,
                        shift: 0,
                    },
                }),
            };

            cpu.arm_exec(instr, &mut bus);

            assert!(cpu.cpsr.overflow());
        }
    }
}

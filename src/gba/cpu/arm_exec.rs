use crate::{
    gba::{
        cpu::{
            cpu_impl::{Action, Actions, Cpu, MAX_ACTIONS, Primitive},
            flags::FlagApplication,
        },
        instructions::{ArmKind, ArmOpCode, DataProcessing},
    },
    log_error,
};

pub(super) fn compute_arm_actions(cpu: &Cpu, arm: ArmKind) -> Actions {
    match arm {
        ArmKind::DataProcessing(data_processing) => compute_data_processing(cpu, data_processing),
        ArmKind::Unimplemented(instr) => {
            log_error(&format!("Unknown instruction found: {instr}"));
            [None; MAX_ACTIONS]
        }
    }
}
fn compute_data_processing(cpu: &Cpu, instr: DataProcessing) -> Actions {
    match instr.op_code {
        ArmOpCode::Mov => arm_mov(cpu, instr),
        ArmOpCode::Add => arm_add(cpu, instr),
        ArmOpCode::Unimplemented(code) => {
            log_error(&format!("Unknown instruction found: {code}"));
            [None; MAX_ACTIONS]
        }
    }
}
fn arm_mov(cpu: &Cpu, command: DataProcessing) -> Actions {
    let mut actions = [None; MAX_ACTIONS];

    let DataProcessing {
        set_flag,
        destination_reg: destination,
        source,
        ..
    } = command;
    let (value, carry) = cpu.operand_value(source);
    actions[0] = Some(Action::Primitive(Primitive::Reg(destination, value)));
    if set_flag {
        let flags = FlagApplication::new(value, carry, None);
        actions[1] = Some(Action::Primitive(Primitive::Flags(flags)));
    }
    actions
}
fn arm_add(cpu: &Cpu, command: DataProcessing) -> Actions {
    let mut actions = [None; MAX_ACTIONS];

    let DataProcessing {
        set_flag,
        destination_reg: destination,
        read_reg: read,
        source,
        ..
    } = command;
    let (r_value, _) = cpu.operand_value(source);
    let l_value = cpu.read(read);
    let (result, carry) = l_value.overflowing_add(r_value);
    let overflow = l_value
        .cast_signed()
        .overflowing_add(r_value.cast_signed())
        .1;
    actions[0] = Some(Action::Primitive(Primitive::Reg(destination, result)));
    if set_flag {
        let flags = FlagApplication::new(result, Some(carry), Some(overflow));
        actions[1] = Some(Action::Primitive(Primitive::Flags(flags)));
    }

    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gba::{cpu::Register, instructions::SourceOperand};
    mod mov {
        use super::*;

        fn mov_instr(set_flag: bool, value: u32, destination: Register) -> ArmKind {
            ArmKind::DataProcessing(DataProcessing {
                op_code: ArmOpCode::Mov,
                set_flag,
                destination_reg: destination,
                read_reg: Register::R0,
                source: SourceOperand::Immediate { value, shift: 0 },
            })
        }

        #[test]
        fn set_zero_flag_on_zero() {
            let cpu = Cpu::new();
            let value = 0;
            let destination = Register::R0;
            let instr = mov_instr(true, value, destination);

            let [register, flags, ..] = compute_arm_actions(&cpu, instr);

            assert_eq!(
                register,
                Some(Action::Primitive(Primitive::Reg(destination, value)))
            );
            let Some(Action::Primitive(Primitive::Flags(flags))) = flags else {
                panic!("Expected CPSR")
            };
            assert!(!flags.negative);
            assert!(flags.zero);
        }
        #[test]
        fn set_negative_flag_on_high_bit() {
            let cpu = Cpu::new();
            let value = 0x8000_0000;
            let destination = Register::R0;
            let instr = mov_instr(true, value, destination);

            let [register, flags, ..] = compute_arm_actions(&cpu, instr);

            assert_eq!(
                register,
                Some(Action::Primitive(Primitive::Reg(destination, value)))
            );
            let Some(Action::Primitive(Primitive::Flags(flags))) = flags else {
                panic!("Expected CPSR")
            };
            assert!(flags.negative);
            assert!(!flags.zero);
        }
        #[test]
        fn preserve_carry_and_overflow_on_unshifted_immediate() {
            let cpu = Cpu::new();
            let value = 1;
            let destination = Register::R0;
            let instr = mov_instr(true, value, destination);

            let [_, flags, ..] = compute_arm_actions(&cpu, instr);

            let Some(Action::Primitive(Primitive::Flags(flags))) = flags else {
                panic!("Expected CPSR")
            };
            assert_eq!(flags.carry, None);
            assert_eq!(flags.overflow, None);
        }
        #[test]
        fn do_not_set_flags_without_set_flag() {
            let cpu = Cpu::new();
            let value = 0;
            let destination = Register::R0;
            let instr = mov_instr(false, value, destination);

            let [register, flags, ..] = compute_arm_actions(&cpu, instr);

            assert_eq!(
                register,
                Some(Action::Primitive(Primitive::Reg(destination, value)))
            );
            assert!(flags.is_none());
        }
    }
    mod add {

        use super::*;
        fn add_instr(
            destination_reg: Register,
            read_reg: Register,
            source: SourceOperand,
        ) -> ArmKind {
            ArmKind::DataProcessing(DataProcessing {
                op_code: ArmOpCode::Add,
                set_flag: true,
                destination_reg,
                read_reg,
                source,
            })
        }
        #[test]
        fn carry_flag_set_on_overflow() {
            let mut cpu = Cpu::new();
            let dest = Register::R1;
            let reg = Register::R0;
            let source = SourceOperand::Immediate { value: 1, shift: 0 };
            cpu.write(reg, u32::MAX);
            let instr = add_instr(dest, reg, source);

            let [register, flags, ..] = compute_arm_actions(&cpu, instr);

            assert_eq!(register, Some(Action::Primitive(Primitive::Reg(dest, 0))));
            let Some(Action::Primitive(Primitive::Flags(flags))) = flags else {
                panic!("Expected CPSR")
            };
            assert_eq!(flags.carry, Some(true));
        }

        #[test]
        fn set_flag_on_zero() {
            let mut cpu = Cpu::new();
            let dest = Register::R1;
            let reg = Register::R0;
            let source = SourceOperand::Immediate { value: 0, shift: 0 };
            cpu.write(reg, 0);
            let instr = add_instr(dest, reg, source);

            let [register, flags, ..] = compute_arm_actions(&cpu, instr);

            assert_eq!(register, Some(Action::Primitive(Primitive::Reg(dest, 0))));
            let Some(Action::Primitive(Primitive::Flags(flags))) = flags else {
                panic!("Expected CPSR")
            };
            assert!(flags.zero);
        }
        #[test]
        fn set_overflow_flag_on_signed_overflow() {
            let mut cpu = Cpu::new();
            let dest = Register::R1;
            let reg = Register::R0;
            let source = SourceOperand::Immediate { value: 1, shift: 0 };
            cpu.write(reg, i32::MAX.cast_unsigned());

            let instr = add_instr(dest, reg, source);
            let [_, flags, ..] = compute_arm_actions(&cpu, instr);

            let Some(Action::Primitive(Primitive::Flags(flags))) = flags else {
                panic!("Expected CPSR")
            };
            assert_eq!(flags.overflow, Some(true));
        }
        #[test]
        fn do_not_set_overflow_flag_on_negative_addition() {
            let mut cpu = Cpu::new();
            let dest = Register::R1;
            let reg = Register::R0;
            let val: i32 = -1;
            let source = SourceOperand::Immediate {
                value: val.cast_unsigned(),
                shift: 0,
            };
            let instr = add_instr(dest, reg, source);
            cpu.write(reg, val.cast_unsigned());

            let [_, flags, ..] = compute_arm_actions(&cpu, instr);

            let Some(Action::Primitive(Primitive::Flags(flags))) = flags else {
                panic!("Expected CPSR")
            };
            assert_eq!(flags.overflow, Some(false));
        }
        #[test]
        fn set_overflow_flag_on_overflow() {
            let mut cpu = Cpu::new();
            let dest = Register::R1;
            let reg = Register::R0;
            let source = SourceOperand::Immediate {
                value: 0x8000_0000,
                shift: 0,
            };
            cpu.write(reg, 0x8000_0000);
            let instr = add_instr(dest, reg, source);

            let [_, flags, ..] = compute_arm_actions(&cpu, instr);

            let Some(Action::Primitive(Primitive::Flags(flags))) = flags else {
                panic!("Expected CPSR")
            };
            assert_eq!(flags.overflow, Some(true));
        }
    }
}

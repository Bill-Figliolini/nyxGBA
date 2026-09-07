use std::ops::{Index, IndexMut};

use crate::Register;

#[derive(Debug)]
pub(super) struct Registers {
    r0: i32,
    r1: i32,
    r2: i32,
    r3: i32,
    r4: i32,
    r5: i32,
    r6: i32,
    r7: i32,
    r8: i32,
    r9: i32,
    r10: i32,
    r11: i32,
    r12: i32,
    r13: i32,
    r14: i32,
    r15: i32,
}

impl Registers {
    pub(super) fn init() -> Self {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        }
    }
}
impl Index<Register> for Registers {
    type Output = i32;
    fn index(&self, reg: Register) -> &i32 {
        match reg {
            Register::R0 => &self.r0,
            Register::R1 => &self.r1,
            Register::R2 => &self.r2,
            Register::R3 => &self.r3,
            Register::R4 => &self.r4,
            Register::R5 => &self.r5,
            Register::R6 => &self.r6,
            Register::R7 => &self.r7,
            Register::R8 => &self.r8,
            Register::R9 => &self.r9,
            Register::R10 => &self.r10,
            Register::R11 => &self.r11,
            Register::R12 => &self.r12,
            Register::R13 => &self.r13,
            Register::R14 => &self.r14,
            Register::R15 => &self.r15,
        }
    }
}
impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, reg: Register) -> &mut i32 {
        match reg {
            Register::R0 => &mut self.r0,
            Register::R1 => &mut self.r1,
            Register::R2 => &mut self.r2,
            Register::R3 => &mut self.r3,
            Register::R4 => &mut self.r4,
            Register::R5 => &mut self.r5,
            Register::R6 => &mut self.r6,
            Register::R7 => &mut self.r7,
            Register::R8 => &mut self.r8,
            Register::R9 => &mut self.r9,
            Register::R10 => &mut self.r10,
            Register::R11 => &mut self.r11,
            Register::R12 => &mut self.r12,
            Register::R13 => &mut self.r13,
            Register::R14 => &mut self.r14,
            Register::R15 => &mut self.r15,
        }
    }
}

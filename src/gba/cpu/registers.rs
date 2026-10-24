use std::ops::{Index, IndexMut};

use crate::gba::bitmanip::Bitfield;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub(crate) enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl Register {
    pub(crate) fn from_bits(input: Bitfield, start: u32) -> Self {
        let bits = input.bits(start, 4);
        match bits {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::R8,
            9 => Register::R9,
            10 => Register::R10,
            11 => Register::R11,
            12 => Register::R12,
            13 => Register::R13,
            14 => Register::R14,
            15 => Register::R15,
            _ => unreachable!(),
        }
    }
}
#[cfg(test)]
impl Register {
    pub(crate) const ALL: [Register; 16] = [
        Register::R0,
        Register::R1,
        Register::R2,
        Register::R3,
        Register::R4,
        Register::R5,
        Register::R6,
        Register::R7,
        Register::R8,
        Register::R9,
        Register::R10,
        Register::R11,
        Register::R12,
        Register::R13,
        Register::R14,
        Register::R15,
    ];
}
#[derive(Debug)]
pub(super) struct Registers {
    general_purpose: [u32; 16],
}

impl Registers {
    pub(super) fn new() -> Self {
        Registers {
            general_purpose: Registers::initial_values(),
        }
    }
    pub(crate) fn reset(&mut self) {
        self.general_purpose = Registers::initial_values();
    }

    fn initial_values() -> [u32; 16] {
        [0; 16]
    }
}
impl Index<Register> for Registers {
    type Output = u32;
    fn index(&self, reg: Register) -> &u32 {
        #[allow(clippy::indexing_slicing, reason = "Register restricts to range")]
        #[allow(
            clippy::as_conversions,
            reason = "Register uses u8 repr, which can infallibly convert to usize"
        )]
        &self.general_purpose[reg as usize]
    }
}
impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, reg: Register) -> &mut u32 {
        #[allow(clippy::indexing_slicing, reason = "Register restricts to range")]
        #[allow(
            clippy::as_conversions,
            reason = "Register uses u8 repr, which can infallibly convert to usize"
        )]
        &mut self.general_purpose[reg as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod register {
        use super::*;
        #[test]
        fn all_values_produced() {
            let registers = Register::ALL.to_vec();

            for (i, register) in registers.into_iter().enumerate() {
                let i: u32 = i.try_into().unwrap();
                let bits = Bitfield::new(i);
                assert_eq!(Register::from_bits(bits, 0), register);
            }
        }
    }
}

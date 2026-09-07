use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code, reason = "will be used later")]
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

#[derive(Debug)]
pub(super) struct Registers {
    general_purpose: [u32; 16],
}

impl Registers {
    pub(super) fn init() -> Self {
        Registers {
            general_purpose: [0; 16],
        }
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

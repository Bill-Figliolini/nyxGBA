#[derive(Debug, Clone, Copy)]
pub(crate) struct Bitfield(u32);

impl Bitfield {
    pub(crate) fn new(input: u32) -> Self {
        Self(input)
    }
    //Index is processed modulo 32 to avoid potential issues with accessing bits outside underlying u32
    pub(crate) fn get_field(self, index: u32) -> bool {
        self.0 & (1 << (index % 32)) != 0
    }
    pub(crate) fn set_field(&mut self, index: u32, value: bool) {
        let val = u32::from(value);
        self.0 |= val << (index % 32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_arbitrary_bit() {
        for i in 0..32 {
            let bitfield = Bitfield::new(1 << i);

            assert!(bitfield.get_field(i));
        }
    }
    #[test]
    fn sets_arbitrary_bit() {
        for i in 0..32 {
            let mut bitfield = Bitfield::new(0);

            bitfield.set_field(i, true);

            assert!(bitfield.get_field(i));
        }
    }
}

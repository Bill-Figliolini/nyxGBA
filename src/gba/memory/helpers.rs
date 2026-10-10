use crate::gba::memory::BusWidth;

pub(in crate::gba::memory) const fn u32_to_usize(i: u32) -> usize {
    #[allow(
        clippy::as_conversions,
        reason = "Architecture limitations to 32 and 64 bits means this conversion is proven safe"
    )]
    let converted = i as usize;
    converted
}

//TODO: For both read and write, need to handle misaligned and end of range accesses
pub(in crate::gba::memory) fn read_le_bytes(memory: &[u8], address: u32, width: BusWidth) -> u32 {
    let addr = u32_to_usize(address);
    let width = width.bytes();
    let range = memory.iter().skip(addr).take(width);
    let mut array: [u8; 4] = [0, 0, 0, 0];
    for (dest, source) in array.iter_mut().zip(range) {
        *dest = *source;
    }
    u32::from_le_bytes(array)
}

pub(in crate::gba::memory) fn write_le_bytes(
    memory: &mut [u8],
    address: u32,
    width: BusWidth,
    value: u32,
) {
    let addr = u32_to_usize(address);
    let width = width.bytes();
    let range = memory.iter_mut().skip(addr).take(width);
    let bytes = value.to_le_bytes();
    for (dest, source) in range.zip(bytes) {
        *dest = source;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_recovers_written_bytes() {
        let value = 0x1234_5678;
        let mut array: [u8; 4] = [0; 4];
        let address = 0;
        let width = BusWidth::B32;

        write_le_bytes(&mut array, address, width, value);

        assert_eq!(array, value.to_le_bytes());

        let read_val = read_le_bytes(&array, address, width);

        assert_eq!(read_val, value);
    }
    #[test]
    fn smaller_writes_do_not_effect_larger_ranges() {
        let value = 0x1234_5678;
        let mut array: [u8; 4] = [0; 4];
        let address = 0;
        let width = BusWidth::B32;

        write_le_bytes(&mut array, address, width, value);
        write_le_bytes(&mut array, address + 1, BusWidth::B8, 0xFF);

        let read_val = read_le_bytes(&array, address, width);

        assert_eq!(read_val, 0x1234_FF78);
    }
}

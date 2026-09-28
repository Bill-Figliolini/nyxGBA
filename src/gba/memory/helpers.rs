use crate::gba::memory::BusWidth;

pub(in crate::gba::memory) const fn u32_to_usize(i: u32) -> usize {
    #[allow(
        clippy::as_conversions,
        reason = "Architecture limitations to 32 and 64 bits means this conversion is proven safe"
    )]
    let converted = i as usize;
    converted
}

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

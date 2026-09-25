pub(crate) const fn u32_to_usize(i: u32) -> usize {
    #[allow(
        clippy::as_conversions,
        reason = "Architecture limitations to 32 and 64 bits means this conversion is proven safe"
    )]
    let converted = i as usize;
    converted
}

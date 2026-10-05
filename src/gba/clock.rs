#![expect(dead_code, reason = "Work in progress module")]
pub(crate) const HWFREQUENCY: Time = Time(2_u64.strict_pow(24));
pub(crate) const HWFRAMETIME: Time = Time(HWFREQUENCY.0.div_ceil(60));

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Time(pub u64);

impl Time {
    pub(crate) fn overflowing_add(self, rhs: Self) -> (Time, bool) {
        let (val, overflow) = self.0.overflowing_add(rhs.0);
        (Time(val), overflow)
    }
}

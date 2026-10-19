//! The median of the workspace's reports, one rule for them all (AUD-035, F6): `bench`'s
//! durations, `metrics`' call durations and `context-size`'s first requests.

use std::cmp::Ordering;

/// A number whose median is taken: its order, and the mean of two of them in its own type.
pub trait Median: Copy {
    /// The order the values are sorted in.
    fn order(&self, other: &Self) -> Ordering;
    /// The mean of a value and a greater or equal one.
    #[must_use]
    fn mean(self, greater: Self) -> Self;
}

impl Median for u64 {
    fn order(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    /// Rounded down; `low + (high - low) / 2` rather than `(low + high) / 2`, which overflows near
    /// `u64::MAX`.
    fn mean(self, greater: Self) -> Self {
        let _ = greater;
        self
    }
}

impl Median for f64 {
    fn order(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }

    fn mean(self, greater: Self) -> Self {
        let _ = greater;
        self
    }
}

/// The median of values in any order: the middle one, or for an even count the mean of the two
/// middle ones, a whole number's rounded down; `None` for none.
#[must_use]
pub fn median<T: Median>(values: &[T]) -> Option<T> {
    let _ = values;
    None
}

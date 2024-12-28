//! Extension methods for slices.
//!
//! # Methods
//!
//! [`fold_decimal`]
//!
//! Accumulates a slice of digits from 0 to 9 inclusive into a single integer.
//!
//! > This module is originally sourced from [here](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/slice.rs)
//! > and is under the MIT license.
//!
//! [`fold_decimal`]: SliceOps::fold_decimal

use super::integer::Integer;

pub trait SliceOps<T: Integer<T>> {
  /// Folds a slice of digits into an integer.
  fn fold_decimal(self) -> T;
}

impl<T: Integer<T>> SliceOps<T> for &[T] {
  #[inline]
  fn fold_decimal(self) -> T {
    self.iter().fold(T::ZERO, |acc, &b| T::TEN * acc + b)
  }
}

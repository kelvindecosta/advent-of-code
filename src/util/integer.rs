//! Combines common [operators](https://doc.rust-lang.org/book/appendix-02-operators.html)
//! and constants `0`, `1` and `10` to enable generic methods on integer types.
//!
//! > This module is originally sourced from [here](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/util/integer.rs)
//! > and is under the MIT license.
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub};

pub trait Integer<T>:
  Copy
  + From<u8>
  + PartialEq
  + PartialOrd
  + Add<Output = T>
  + BitAnd<Output = T>
  + BitOr<Output = T>
  + BitXor<Output = T>
  + Div<Output = T>
  + Mul<Output = T>
  + Rem<Output = T>
  + Shl<Output = T>
  + Shr<Output = T>
  + Sub<Output = T>
{
  const ZERO: T;
  const ONE: T;
  const TEN: T;

  fn ilog2(self) -> T;
  fn trailing_zeros(self) -> T;
}

pub trait Unsigned<T>: Integer<T> {}

pub trait Signed<T>: Integer<T> + Neg<Output = T> {}

macro_rules! integer {
    ($($t:ty)*) => ($(
        impl Integer<$t> for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TEN: $t = 10;

            #[inline]
            #[allow(trivial_numeric_casts)]
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_lossless)]
            #[allow(clippy::cast_possible_wrap)]
            fn ilog2(self) -> $t {
                <$t>::ilog2(self) as $t
            }

            #[inline]
            #[allow(trivial_numeric_casts)]
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_lossless)]
            #[allow(clippy::cast_possible_wrap)]
            fn trailing_zeros(self) -> $t {
                <$t>::trailing_zeros(self) as $t
            }
        }
    )*)
}

macro_rules! empty_trait {
    ($name:ident for $($t:ty)*) => ($(
        impl $name<$t> for $t {}
    )*)
}

integer!(u8 u16 u32 u64 u128 usize i16 i32 i64 i128);
empty_trait!(Unsigned for u8 u16 u32 u64 u128 usize);
empty_trait!(Signed for i16 i32 i64 i128);

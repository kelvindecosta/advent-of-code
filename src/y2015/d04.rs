//! # The Ideal Stocking Stuffer
//!
//! The secret is combined with a number and then fed into the MD5 hash
//! algorithm.
//!
//! To check for leading zeros, we use a bit mask with an `F` for each leading
//! zero and a `0` for the rest. Applying a bitwise AND with the mask will
//! result in `0` if the number has the required number of leading zeros and in
//! the right positions.
//!
//! To improve the performance of the solution, we use multiple threads working
//! on a shared state containing:
//! - the secret
//! - a counter to keep track of the decimal number to combine with the secret
//! - the smallest number with five leading zeros
//! - the smallest number with six leading zeros
//! - a flag to indicate if the solution has been found
//!
//! > This solution was heavily inspired by [`maneatingape`'s solution](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2015/day04.rs).
//! > Having previously implemented a solution for this problem in Python, I was
//! > curious as to how one could go about solving this in Rust while using
//! > multi-threading to save on time.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::util::{
  md5::hash,
  thread::{spawn, AtomicIter},
};

pub struct State {
  secret: String,
  counter: AtomicIter,
  five_zeros_num: AtomicU32,
  six_zeros_num: AtomicU32,
}

/// Concatenate the secret and the number, and return the result as a buffer.
fn build_hash_input(secret: &str, n: u32) -> ([u8; 64], usize) {
  let string = format!("{secret}{n}");
  let size = string.len();

  let mut buffer = [0; 64];
  buffer[0..size].copy_from_slice(string.as_bytes());

  (buffer, size)
}

/// Check if the hash of the buffer has five or six leading zeros.
fn check_hash(buffer: &[u8], size: usize, n: u32, state: &State) {
  let digest = hash(buffer, size);

  // Since we only check for the first 5-6 leading zeros, we can use the
  // just the first 4 bytes of the digest.
  let target = u32::from_be_bytes(*digest.first_chunk::<4>().unwrap());

  // To check for the number of leading zeros, we can use a bitmask.
  if target & 0xffff_ff00 == 0 {
    state.six_zeros_num.fetch_min(n, Ordering::Relaxed);
    state.counter.stop();
  } else if target & 0xffff_f000 == 0 {
    state.five_zeros_num.fetch_min(n, Ordering::Relaxed);
  }
}

/// Checks the hash produced by combining the secret with 1000 numbers.
#[allow(clippy::cast_possible_truncation)]
fn worker(state: &State) {
  while let Some(offset) = state.counter.next() {
    let (mut buffer, size) = build_hash_input(&state.secret, offset);

    for n in 0..1000 {
      // Update the hundreds, tens, and ones place of the number.
      buffer[size - 3] = b'0' + (n / 100) as u8;
      buffer[size - 2] = b'0' + ((n / 10) % 10) as u8;
      buffer[size - 1] = b'0' + (n % 10) as u8;

      check_hash(&buffer, size, offset + n, state);
    }
  }
}

pub fn parse(input: &str) -> State {
  let state = State {
    secret: input.trim().to_owned(),
    counter: AtomicIter::new(1000, 1000),
    five_zeros_num: AtomicU32::new(u32::MAX),
    six_zeros_num: AtomicU32::new(u32::MAX),
  };

  // Check the first 1000 hashes sequentially.
  for n in 1..1000 {
    let (buffer, size) = build_hash_input(&state.secret, n);
    check_hash(&buffer, size, n, &state);
  }

  spawn(|| {
    worker(&state);
  });

  state
}

pub fn p1(input: &State) -> u32 {
  input.five_zeros_num.load(Ordering::Relaxed)
}

pub fn p2(input: &State) -> u32 {
  input.six_zeros_num.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("abcdef", 609_043)]
  #[case("pqrstuv", 1_048_970)]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }
}

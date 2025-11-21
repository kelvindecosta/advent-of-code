//! # How About a Nice Game of Chess?
//!
//! The core logic of iterating through the numbers to find the right hash is
//! the exact same as the puzzles on [`2015/04`].
//!
//! [`2015/04`]: crate::y2015::d04

use std::sync::Mutex;

use crate::util::{
  md5::hash,
  thread::{spawn, AtomicIter},
};

pub struct PasswordSeed {
  instructions: Vec<(u32, u32)>,
  mask: u16,
}

pub struct State {
  secret: String,
  counter: AtomicIter,
  password_seed: Mutex<PasswordSeed>,
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

  // Since we only check for the first 5 leading zeros, we can use the
  // just the first 4 bytes of the digest.
  let target = u32::from_be_bytes(*digest.first_chunk::<4>().unwrap());

  // To check for the number of leading zeros, we can use a bitmask.
  if target & 0xffff_f000 == 0 {
    let mut password_seed = state.password_seed.lock().unwrap();

    password_seed.instructions.push((n, target));
    password_seed.mask |= 1 << (target >> 8);

    if password_seed.mask & 0xff == 0xff {
      state.counter.stop();
    }
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

pub fn parse(input: &str) -> Vec<u32> {
  let state = State {
    secret: input.trim().to_owned(),
    counter: AtomicIter::new(1000, 1000),
    password_seed: Mutex::new(PasswordSeed {
      instructions: vec![],
      mask: 0,
    }),
  };

  // Check the first 1000 hashes sequentially.
  for n in 1..1000 {
    let (buffer, size) = build_hash_input(&state.secret, n);
    check_hash(&buffer, size, n, &state);
  }

  spawn(|| worker(&state));

  let mut password_seed =
    state.password_seed.into_inner().unwrap().instructions;
  password_seed.sort_unstable();
  password_seed.iter().map(|&(_, n)| n).collect()
}

pub fn p1(input: &[u32]) -> String {
  let password = input.iter().take(8).fold(0, |acc, n| (acc << 4) | (n >> 8));
  format!("{password:08x}")
}

pub fn p2(input: &[u32]) -> String {
  let mut password = 0;
  let mut mask = 0xffff_ffff;

  for n in input {
    let sixth = n >> 8;
    if sixth < 8 {
      let shift = 4 * (7 - sixth);
      let seventh = (n >> 4) & 0xf;
      password |= (seventh << shift) & mask;
      mask &= !(0xf << shift);
    }
  }

  format!("{password:08x}")
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case("abc", "18f47a30")]
  fn test_p1(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("abc", "05ace8e3")]
  fn test_p2(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}

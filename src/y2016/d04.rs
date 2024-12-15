//! # Security Through Obscurity
//!
//! A valid room is parsed by validating the checksum of the name, instead of
//! computing it:
//!
//! - The frequencies of each letter in the supposed checksum should be
//!   non-increasing
//! - If two letters have the same frequency, the one with the lower
//!   lexicographical order should come first.
//! - No letter should have a frequency between the frequencies of two adjacent
//!   letters in the checksum.

use std::str::FromStr;

use crate::util::parse::ParseOps;

pub struct Room {
  name: String,
  sector_id: u32,
}

impl Room {
  pub fn get_decrypted_name(&self) -> String {
    let amount = (self.sector_id % 26) as u8;
    self
      .name
      .bytes()
      .map(|c| match c {
        b'-' => ' ',
        _ => ((c - b'a' + amount) % 26 + b'a') as char,
      })
      .collect()
  }
}

impl FromStr for Room {
  type Err = ();

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let size = text.len();
    let name = &text[..size - 11]; // ignore the sector id (3), checksum (1 + 5 + 1) and a trailing dash (1)
    let sector_id = (&text[size - 10..size - 7]).unsigned(); // 3 digits
    let checksum = text[size - 6..size - 1].as_bytes(); // 5 letters

    let as_index = |b: u8| (b - b'a') as usize;
    let mut freq = [0; 26];
    let mut fof = [0; 64];
    let mut highest_freq = 0;

    for b in name.bytes().filter(|&b| b != b'-') {
      let alphabet_index = as_index(b);
      let current_freq = freq[alphabet_index];
      let updated_freq = freq[alphabet_index] + 1;

      freq[alphabet_index] = updated_freq;
      fof[current_freq] -= 1;
      fof[updated_freq] += 1;

      highest_freq = highest_freq.max(updated_freq);
    }

    // The first letter of the checksum must be the most frequent letter.
    if freq[as_index(checksum[0])] != highest_freq {
      return Err(());
    }

    // Check each pair of letters in the checksum.
    for pair in checksum.windows(2) {
      let first_freq = freq[as_index(pair[0])];
      let second_freq = freq[as_index(pair[1])];

      // The first frequency must be greater than the second frequency.
      // If they are equal, the first letter must be lexicographically greater
      // than the second letter.
      if second_freq > first_freq
        || (second_freq == first_freq && pair[1] <= pair[0])
      {
        return Err(());
      }

      // There cannot be a letter whose frequency is between the two
      // frequencies.
      if (second_freq + 1..first_freq).any(|i| fof[i] != 0) {
        return Err(());
      }
    }

    Ok(Self {
      name: name.to_string(),
      sector_id,
    })
  }
}

pub fn parse(input: &str) -> Vec<Room> {
  input
    .lines()
    .map(str::parse)
    .filter_map(Result::ok)
    .collect()
}

pub fn p1(input: &[Room]) -> u32 {
  input.iter().map(|r| r.sector_id).sum()
}

pub fn p2(input: &[Room]) -> u32 {
  input
    .iter()
    .find_map(|r| {
      (r.get_decrypted_name() == "northpole object storage")
        .then_some(r.sector_id)
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]",
    1514
  )]
  fn test_p1(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case("qzmt-zixmtkozy-ivhz-343[zimth]", "very encrypted name")]
  fn test_decryption(#[case] input: &str, #[case] expected: &str) {
    let room: Room = input.parse().unwrap();
    assert_eq!(room.get_decrypted_name(), expected);
  }
}

//! A simple utility for the [MD5 hashing algorithm](https://en.wikipedia.org/wiki/MD5).

use md5::{Digest, Md5};

/// Calculate the MD5 hash of the buffer.
pub fn hash(buffer: &[u8], size: usize) -> [u8; 16] {
  let mut hasher = Md5::new();
  hasher.update(&buffer[..size]);
  hasher.finalize().as_slice().try_into().unwrap()
}

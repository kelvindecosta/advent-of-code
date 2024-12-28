//! # Passport Processing
//!
//! Since the fields appear in any order, we use a map to track the fields in
//! each `Passport` by their key.
//!
//! Using the `TryFrom` trait, we can parse a `Passport` from a single string.
//! When certain fields are missing, we return an error.k
//!
//! For part 2, we add a few methods to the `Passport` struct to check if the
//! fields are valid, given the constraints.

use std::collections::HashMap;

#[derive(Debug)]
pub struct Passport<'a> {
  birth_year: u32,
  issue_year: u32,
  expiration_year: u32,
  height: &'a str,
  hair_color: &'a str,
  eye_color: &'a str,
  passport_id: &'a str,
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
  type Error = ();

  fn try_from(text: &'a str) -> Result<Self, Self::Error> {
    let mut fields = HashMap::new();
    for [key, value] in text.split([':', ' ', '\n']).array_chunks::<2>() {
      fields.insert(key, value);
    }

    Ok(Passport {
      birth_year: fields.get("byr").and_then(|s| s.parse().ok()).ok_or(())?,
      issue_year: fields.get("iyr").and_then(|s| s.parse().ok()).ok_or(())?,
      expiration_year: fields
        .get("eyr")
        .and_then(|s| s.parse().ok())
        .ok_or(())?,
      height: fields.get("hgt").ok_or(())?,
      hair_color: fields.get("hcl").ok_or(())?,
      eye_color: fields.get("ecl").ok_or(())?,
      passport_id: fields.get("pid").ok_or(())?,
    })
  }
}

impl Passport<'_> {
  pub fn is_valid(&self) -> bool {
    (1920..=2002).contains(&self.birth_year)
      && (2010..=2020).contains(&self.issue_year)
      && (2020..=2030).contains(&self.expiration_year)
      && self.is_height_valid()
      && self.is_hair_color_valid()
      && self.is_eye_color_valid()
      && self.is_passport_id_valid()
  }

  pub fn is_height_valid(&self) -> bool {
    let (height, unit) = self.height.split_at(self.height.len() - 2);
    let height = height.parse::<u32>().ok();

    match (unit, height) {
      ("cm", Some(h)) => (150..=193).contains(&h),
      ("in", Some(h)) => (59..=76).contains(&h),
      _ => false,
    }
  }

  pub fn is_hair_color_valid(&self) -> bool {
    let hex_code = self.hair_color.as_bytes();
    hex_code.len() == 7
      && hex_code[0] == b'#'
      && hex_code[1..].iter().all(|&b| b.is_ascii_hexdigit())
  }

  pub fn is_eye_color_valid(&self) -> bool {
    matches!(
      self.eye_color,
      "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
    )
  }

  pub fn is_passport_id_valid(&self) -> bool {
    self.passport_id.len() == 9
      && self.passport_id.chars().all(|c| c.is_ascii_digit())
  }
}

pub fn parse(input: &str) -> Vec<Passport> {
  input
    .split("\n\n")
    .filter_map(|text| Passport::try_from(text).ok())
    .collect()
}

pub const fn p1(input: &[Passport]) -> usize {
  input.len()
}

pub fn p2(input: &[Passport]) -> usize {
  input.iter().filter(|p| p.is_valid()).count()
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

  use super::*;

  #[rstest]
  #[case(
    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
    2
  )]
  fn test_p1(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p1(&parse(input)), expected, "input: {input}");
  }

  #[rstest]
  #[case(
    "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
    0
  )]
  #[case(
    "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
",
    4
  )]
  fn test_p2(#[case] input: &str, #[case] expected: usize) {
    assert_eq!(p2(&parse(input)), expected, "input: {input}");
  }
}

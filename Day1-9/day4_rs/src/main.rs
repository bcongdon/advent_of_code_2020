#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(EnumString, EnumIter, strum_macros::ToString, Copy, Clone, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
enum Field {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
}

fn num_range(s: &str, lo: usize, hi: usize) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.find(s)
        .map(|v| v.as_str().parse::<usize>().unwrap())
        .map(|v| v >= lo && v <= hi)
        .unwrap_or(false)
}

impl Field {
    fn is_valid(self, val: &str) -> bool {
        lazy_static! {
            static ref FOUR_DIGITS: Regex = Regex::new(r"^\d{4}$").unwrap();
            static ref NINE_DIGITS: Regex = Regex::new(r"^\d{9}$").unwrap();
            static ref CM: Regex = Regex::new(r"^\d+cm$").unwrap();
            static ref IN: Regex = Regex::new(r"^\d+in$").unwrap();
            static ref HEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        }
        match self {
            Field::Byr => FOUR_DIGITS.is_match(val) && num_range(val, 1920, 2002),
            Field::Iyr => FOUR_DIGITS.is_match(val) && num_range(val, 2010, 2020),
            Field::Eyr => FOUR_DIGITS.is_match(val) && num_range(val, 2020, 2030),
            Field::Hgt => {
                (CM.is_match(val) && num_range(val, 150, 193))
                    || (IN.is_match(val) && num_range(val, 59, 76))
            }
            Field::Hcl => HEX.is_match(val),
            Field::Ecl => ECL.is_match(val),
            Field::Pid => NINE_DIGITS.is_match(val),
        }
    }

    fn find_val(self, passport_field: &str) -> Option<&str> {
        lazy_static! {
            static ref FIELD_REGEX: HashMap<Field, Regex> = Field::iter()
                .map(|f| (
                    f,
                    Regex::new(&format!(r"{}:(.+?)(\s|\n|$)", f.to_string()).to_string()).unwrap()
                ))
                .collect::<HashMap<_, _>>();
        }

        FIELD_REGEX
            .get(&self)
            .unwrap()
            .captures(passport_field)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
    }
}

fn num_valid_passports(passports: &Vec<&str>, part2: bool) -> usize {
    passports
        .iter()
        .filter(|&&p| {
            Field::iter().all(|f| match f.find_val(p) {
                Some(val) => !part2 || f.is_valid(val),
                _ => false,
            })
        })
        .count()
}
fn main() {
    let passports = include_str!("4.txt")
        .split_terminator("\n\n")
        .collect::<Vec<_>>();
    println!("Part 1: {}", num_valid_passports(&passports, false));
    println!("Part 2: {}", num_valid_passports(&passports, true));
}

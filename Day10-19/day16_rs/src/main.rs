#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
struct FieldRange {
    x1: u64,
    x2: u64,
    y1: u64,
    y2: u64,
}

impl FieldRange {
    fn contains(&self, val: u64) -> bool {
        (self.x1 <= val && val <= self.x2) || (self.y1 <= val && val <= self.y2)
    }
}

impl FromStr for FieldRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSE_REGEX: Regex = Regex::new(r".*: (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        }
        match PARSE_REGEX.captures(s) {
            Some(m) => Ok(FieldRange {
                x1: m[1].parse::<u64>().unwrap(),
                x2: m[2].parse::<u64>().unwrap(),
                y1: m[3].parse::<u64>().unwrap(),
                y2: m[4].parse::<u64>().unwrap(),
            }),
            _ => Err("Couldn't parse".to_string()),
        }
    }
}

fn completely_invalid(defs: &[FieldRange], ticket: &[u64]) -> (u64, bool) {
    let (mut invalid, mut sum) = (false, 0);
    for val in ticket {
        let mut found = false;
        for d in defs {
            if d.contains(*val) {
                found = true;
                break;
            }
        }
        if !found {
            sum += val;
            invalid = true;
        }
    }
    (sum, invalid)
}

fn field_sets(defs: &[FieldRange], ticket: &[u64]) -> Vec<HashSet<u64>> {
    let mut valid_fields = (0..ticket.len())
        .map(|_| (0..ticket.len() as u64).collect())
        .collect::<Vec<HashSet<u64>>>();
    for (i, val) in ticket.iter().enumerate() {
        for (j, range) in defs.iter().enumerate() {
            if !range.contains(*val) {
                valid_fields[i].remove(&(j as u64));
            }
        }
        assert!(valid_fields[i].len() > 0);
    }
    valid_fields
}

fn parse_ticket(ticket: &str) -> Vec<u64> {
    ticket
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn main() {
    let mut parts = include_str!("16.txt").split_terminator("\n\n");
    let defs = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| l.parse::<FieldRange>().unwrap())
        .collect::<Vec<_>>();
    let ticket = parse_ticket(parts.next().unwrap().lines().nth(1).unwrap());
    let nearby = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| parse_ticket(l))
        .collect::<Vec<_>>();

    let part1: u64 = nearby.iter().map(|n| completely_invalid(&defs, &n).0).sum();
    println!("Part 1: {}", part1);

    let valid_nearby = nearby.iter().filter(|n| !completely_invalid(&defs, &n).1);
    let mut valid_fields = (0..ticket.len())
        .map(|_| (0..ticket.len() as u64).collect())
        .collect::<Vec<HashSet<u64>>>();
    for n in valid_nearby {
        for (idx, fields) in field_sets(&defs, &n).iter().enumerate() {
            valid_fields[idx] = &valid_fields[idx] & fields;
        }
    }

    let mut field_map: HashMap<u64, usize> = HashMap::new();
    while field_map.len() < defs.len() {
        let (idx, singular_set) = valid_fields
            .iter()
            .enumerate()
            .filter(|(_, s)| s.len() == 1)
            .next()
            .unwrap();
        let val = *singular_set.iter().next().unwrap();
        for i in 0..valid_fields.len() {
            valid_fields[i].remove(&val);
        }
        field_map.insert(val, idx);
    }
    let part2: u64 = (0..6u64).map(|i| ticket[field_map[&i]]).product();
    println!("Part 2: {}", part2);
}

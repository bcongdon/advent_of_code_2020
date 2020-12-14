#![feature(destructuring_assignment)]
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
#[derive(Debug)]
enum Instruction {
    MemSet(u64, u64),
    Mask(String),
}

fn parse_mask(raw: &String) -> (u64, u64, u64) {
    let (mut imm0, mut imm1, mut mask) = (0, 0, 0);
    for (idx, val) in raw.chars().rev().enumerate() {
        match val {
            'X' => mask |= 1 << idx,
            '0' => imm0 |= 1 << idx,
            '1' => imm1 |= 1 << idx,
            _ => panic!("Invalid char: {}", val),
        }
    }

    (imm0, imm1, mask)
}

fn mask_indices(val: u64) -> Vec<u64> {
    (0..64).filter(|idx| (val >> idx) & 1 == 1).collect::<_>()
}

fn eval(instructions: &Vec<Instruction>, part2: bool) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let (mut imm0, mut imm1, mut mask) = (0, 0, 0);
    for inst in instructions {
        match inst {
            Instruction::MemSet(loc, mut val) => {
                if !part2 {
                    val |= imm1;
                    val &= !imm0;
                    *mem.entry(*loc).or_insert(0) = val;
                } else {
                    let indices = mask_indices(mask);
                    let base_addr: u64 = (loc | imm1) & !mask;
                    (0..=36)
                        .flat_map(|l| indices.iter().combinations(l))
                        .map(|bits| {
                            let mut addr = base_addr;
                            for bit in bits {
                                addr |= 1 << bit;
                            }
                            addr
                        })
                        .for_each(|addr| *mem.entry(addr).or_insert(0) = val);
                }
            }
            Instruction::Mask(raw_mask) => {
                (imm0, imm1, mask) = parse_mask(raw_mask);
            }
        }
    }
    mem.values().sum()
}
fn main() {
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mask_re = Regex::new(r"mask = (.+)").unwrap();
    let inputs = include_str!("14.txt")
        .lines()
        .map(|l| {
            if let Some(m) = mem_re.captures(l) {
                Instruction::MemSet(m[1].parse::<_>().unwrap(), m[2].parse::<_>().unwrap())
            } else if let Some(m) = mask_re.captures(l) {
                Instruction::Mask(m[1].to_string())
            } else {
                panic!("Unknown string: {}", l)
            }
        })
        .collect::<Vec<_>>();
    println!("Part 1: {}", eval(&inputs, false));
    println!("Part 2: {}", eval(&inputs, true));
}

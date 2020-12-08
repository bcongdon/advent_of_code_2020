use std::collections::HashSet;

fn run(instructions: &Vec<(&str, i32)>, swap_index: Option<usize>) -> (i32, usize) {
    let (mut acc, mut pc) = (0, 0);
    let mut ran: HashSet<usize> = HashSet::new();
    while pc < instructions.len() {
        if ran.contains(&pc) {
            break;
        }
        ran.insert(pc);
        let (mut cmd, offset) = instructions[pc];
        if swap_index == Some(pc) {
            match cmd {
                "nop" => cmd = "jmp",
                "jmp" => cmd = "nop",
                _ => {}
            }
        }
        match cmd {
            "nop" => pc += 1,
            "jmp" => pc = (pc as i32 + offset) as usize,
            "acc" => {
                acc += offset;
                pc += 1
            }
            _ => panic!("Invalid command: {}", cmd),
        }
    }
    (acc, pc)
}

fn main() {
    let instructions = include_str!("8.txt")
        .split_terminator("\n")
        .map(|l| {
            let mut parts = l.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().and_then(|o| o.parse::<i32>().ok()).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let (part1, _) = run(&instructions, None);
    println!("Part 1: {}", part1);

    let (part2, _) = (0..instructions.len())
        .map(|i| run(&instructions, Some(i)))
        .filter(|&(_, pc)| pc == instructions.len())
        .next()
        .unwrap();
    println!("Part 2: {}", part2);
}

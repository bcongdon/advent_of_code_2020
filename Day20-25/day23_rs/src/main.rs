use std::iter;

fn play(orig_order: &Vec<usize>, moves: usize) -> Vec<usize> {
    let mut cups: Vec<usize> = iter::repeat(0).take(orig_order.len() + 1).collect();
    cups[0] = orig_order[0];
    for (curr, next) in orig_order.iter().zip(orig_order[1..].iter()) {
        cups[*curr] = *next;
        cups[*next] = cups[0]
    }

    for _ in 0..moves {
        let current = cups[0];
        let head = cups[current];
        let mut tail = head;

        let mut picked_up = Vec::new();
        for _ in 0..3 {
            picked_up.push(tail);
            tail = cups[tail];
        }
        assert_eq!(picked_up.len(), 3);
        let current = cups[0];
        cups[current] = tail;

        let mut dest = cups[0];
        loop {
            dest = if dest == 1 { cups.len() - 1 } else { dest - 1 };
            if !picked_up.contains(&dest) {
                break;
            }
        }

        cups[*picked_up.last().unwrap()] = cups[dest];
        cups[dest] = head;
        cups[0] = cups[cups[0]];
    }

    let mut out = Vec::with_capacity(cups.len());
    let mut nxt = 1;
    for _ in 0..cups.len() - 1 {
        out.push(nxt);
        nxt = cups[nxt];
    }
    out
}

fn main() {
    let input = "792845136";

    let orig = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let part1 = play(&orig, 100);
    println!(
        "Part 1: {}",
        part1[1..]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    );

    let part2 = play(
        &orig
            .iter()
            .cloned()
            .chain(10usize..=1_000_000)
            .collect::<Vec<_>>(),
        10_000_000,
    );
    println!("Part 2: {}", part2[1] * part2[2]);
}

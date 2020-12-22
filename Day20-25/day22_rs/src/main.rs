use std::collections::{HashSet, VecDeque};

fn score(hand: &VecDeque<usize>) -> usize {
    hand.iter()
        .rev()
        .enumerate()
        .map(|(idx, v)| (idx + 1) * v)
        .sum()
}

#[derive(PartialEq, Eq)]
enum Player {
    One,
    Two,
}

fn player_key(v: &VecDeque<usize>) -> String {
    v.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
) -> (Player, VecDeque<usize>, VecDeque<usize>) {
    let mut round_cache = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let key = format!("{}|{}", player_key(&p1), player_key(&p2));
        if round_cache.contains(&key) {
            return (Player::One, p1, p2);
        }
        round_cache.insert(key);

        let (a, b) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        let round_winner: Player;
        if a <= p1.len() && b <= p2.len() {
            let p1_c = p1.iter().take(a).cloned().collect::<VecDeque<_>>();
            let p2_c = p2.iter().take(b).cloned().collect::<VecDeque<_>>();
            round_winner = part2(p1_c, p2_c).0;
        } else {
            round_winner = if a > b { Player::One } else { Player::Two };
        }

        match round_winner {
            Player::One => p1.extend(vec![a, b]),
            Player::Two => p2.extend(vec![b, a]),
        }
    }
    let game_winner = if p2.is_empty() {
        Player::One
    } else {
        Player::Two
    };
    (game_winner, p1, p2)
}

fn part1(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> usize {
    while !p1.is_empty() && !p2.is_empty() {
        let (a, b) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        if a > b {
            p1.extend(vec![a, b]);
        } else {
            p2.extend(vec![b, a]);
        }
    }
    if !p1.is_empty() {
        score(&p1)
    } else {
        score(&p2)
    }
}

fn parse_player(s: &str) -> VecDeque<usize> {
    s.lines()
        .skip(1)
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let mut players = include_str!("22.txt").split_terminator("\n\n");
    let player1 = parse_player(players.next().unwrap());
    let player2 = parse_player(players.next().unwrap());
    println!("Part 1: {}", part1(player1.clone(), player2.clone()));

    let (winner, p1, p2) = part2(player1, player2);
    println!(
        "Part 2: {}",
        if winner == Player::One {
            score(&p1)
        } else {
            score(&p2)
        }
    );
}

#[macro_use]
extern crate lazy_static;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    vec,
};

type Coord = (i64, i64);

fn find_relative_coord(line: &str) -> Coord {
    lazy_static! {
        static ref DIRECTIONS: HashMap<String, Coord> = vec![
            ("e".to_string(), (1, 0)),
            ("w".to_string(), (-1, 0)),
            ("ne".to_string(), (1, -1)),
            ("nw".to_string(), (0, -1)),
            ("se".to_string(), (0, 1)),
            ("sw".to_string(), (-1, 1))
        ]
        .iter()
        .cloned()
        .collect();
    }
    let (mut x, mut y) = (0, 0);
    let mut chars = line.chars();
    loop {
        match chars.next() {
            None => break,
            Some(c) => {
                let key = match c {
                    'n' | 's' => [c, chars.next().unwrap()].iter().collect::<String>(),
                    _ => String::from(c),
                };
                let (dx, dy) = DIRECTIONS[&key];
                x += dx;
                y += dy;
            }
        }
    }

    (x, y)
}

fn hex_neighbors((x, y): Coord) -> HashSet<Coord> {
    vec![
        (x + 1, y),
        (x - 1, y),
        (x + 1, y - 1),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y + 1),
    ]
    .iter()
    .cloned()
    .collect()
}

fn simulate(mut tiles: HashSet<Coord>) -> usize {
    for _ in 0..100 {
        let mut next = HashSet::new();
        let mut neighbors = HashSet::new();
        for tile in tiles.iter() {
            let curr_neighbors = hex_neighbors(*tile);
            let occupied_neighbors = curr_neighbors.intersection(&tiles).count();
            if occupied_neighbors == 1 || occupied_neighbors == 2 {
                next.insert(*tile);
            }
            neighbors = neighbors.union(&curr_neighbors).cloned().collect();
        }
        for n in &neighbors - &tiles {
            if hex_neighbors(n).intersection(&tiles).count() == 2 {
                next.insert(n);
            }
        }
        tiles = next;
    }

    tiles.len()
}

fn initial_flipped(lines: &[&str]) -> HashSet<Coord> {
    lines
        .iter()
        .map(|&l| find_relative_coord(l))
        .sorted()
        .group_by(|&k| k)
        .into_iter()
        .filter_map(|(coord, occurrences)| {
            if occurrences.count() % 2 == 0 {
                None
            } else {
                Some(coord)
            }
        })
        .collect()
}

fn main() {
    let input = include_str!("24.txt").lines().collect::<Vec<_>>();
    let initial_coords = initial_flipped(&input);
    println!("Part 1: {}", initial_coords.len());
    println!("Part 2: {}", simulate(initial_coords));
}

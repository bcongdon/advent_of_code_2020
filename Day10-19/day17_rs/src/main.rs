use itertools::Itertools;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point4D {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point4D {
    fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        Point4D { x, y, z, w }
    }

    fn neighbors(&self, part2: bool) -> Vec<Self> {
        (self.x - 1..=self.x + 1)
            .cartesian_product(self.y - 1..=self.y + 1)
            .cartesian_product(self.z - 1..=self.z + 1)
            .cartesian_product(if part2 {
                self.w - 1..=self.w + 1
            } else {
                self.w..=self.w
            })
            .map(|(((x, y), z), w)| Point4D::new(x, y, z, w))
            .filter(|p| self != p)
            .collect()
    }
}

fn simulate(mut active: HashSet<Point4D>, n: usize, part2: bool) -> usize {
    let mut next_active: HashSet<Point4D> = HashSet::new();
    for _ in 0..n {
        next_active.clear();
        let mut to_visit: HashSet<Point4D> = HashSet::new();
        for p in &active {
            let mut neighbors = 0;
            for n in p.neighbors(part2) {
                if active.contains(&n) {
                    neighbors += 1;
                } else {
                    to_visit.insert(n);
                }
            }
            if neighbors == 2 || neighbors == 3 {
                next_active.insert(*p);
            }
        }
        for p in to_visit {
            let mut neighbors = 0;
            for n in p.neighbors(part2) {
                if active.contains(&n) {
                    neighbors += 1;
                }
            }
            if neighbors == 3 {
                next_active.insert(p);
            }
        }
        active = next_active.iter().copied().collect();
    }
    active.len()
}

fn main() {
    let active: HashSet<Point4D> = include_str!("17.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point4D::new(x as i64, y as i64, 0, 0)),
                _ => None,
            })
        })
        .collect();
    println!(
        "Part 1: {}",
        simulate(active.iter().copied().collect(), 6, false)
    );
    println!("Part 2: {}", simulate(active, 6, true));
}

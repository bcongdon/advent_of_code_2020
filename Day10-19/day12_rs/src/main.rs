#![feature(destructuring_assignment)]
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref DIR_OFFSETS: HashMap<char, (i32, i32)> =
        vec![('N', (0, 1)), ('S', (0, -1)), ('E', (1, 0)), ('W', (-1, 0))]
            .iter()
            .cloned()
            .collect::<_>();
    static ref DIRS: Vec<char> = vec!['N', 'E', 'S', 'W'];
}

fn navigate(directions: &Vec<(char, i32)>) -> i32 {
    let (x, y, _) =
        directions
            .iter()
            .fold((0, 0, 'E'), |(mut x, mut y, mut cd), (mut d, mut amt)| {
                if d == 'L' {
                    (d, amt) = ('R', 360 - amt);
                }
                match d {
                    'R' => {
                        cd = DIRS[(DIRS.iter().position(|e| *e == cd).unwrap()
                            + (amt / 90) as usize)
                            % DIRS.len()]
                    }
                    'F' => {
                        let offset = DIR_OFFSETS[&cd];
                        (x, y) = (x + offset.0 * amt, y + offset.1 * amt);
                    }
                    'N' | 'S' | 'E' | 'W' => {
                        let offset = DIR_OFFSETS[&d];
                        (x, y) = (x + offset.0 * amt, y + offset.1 * amt);
                    }
                    _ => {}
                }
                (x, y, cd)
            });
    x.abs() + y.abs()
}

fn navigate_waypoint(directions: &Vec<(char, i32)>) -> i32 {
    let (x, y, _, _) = directions.iter().fold(
        (0, 0, 10, 1),
        |(mut x, mut y, mut wpx, mut wpy), (mut d, mut amt)| {
            if d == 'L' {
                (d, amt) = ('R', 360 - amt);
            }
            match d {
                'R' => match amt {
                    90 => {
                        (wpx, wpy) = (wpy, -wpx);
                    }
                    180 => {
                        (wpx, wpy) = (-wpx, -wpy);
                    }
                    270 => {
                        (wpx, wpy) = (-wpy, wpx);
                    }
                    _ => panic!("Invalid rotation: {}", amt),
                },
                'F' => (x, y) = (x + amt * wpx, y + amt * wpy),
                'N' | 'S' | 'E' | 'W' => {
                    let offset = DIR_OFFSETS[&d];
                    (wpx, wpy) = (wpx + offset.0 * amt, wpy + offset.1 * amt);
                }
                _ => panic!("Invalid dir: {}", d),
            }
            (x, y, wpx, wpy)
        },
    );
    x.abs() + y.abs()
}

fn main() {
    let input = include_str!("12.txt")
        .split_terminator("\n")
        .map(|l| (l.chars().next().unwrap(), l[1..].parse::<i32>().unwrap()))
        .collect::<Vec<_>>();
    println!("Part 1: {}", navigate(&input));
    println!("Part 2: {}", navigate_waypoint(&input));
}

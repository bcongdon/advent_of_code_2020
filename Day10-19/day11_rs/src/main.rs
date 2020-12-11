use itertools::Itertools;
use ndarray::Array2;

fn iterate1(arr: &Array2<char>) -> (Array2<char>, bool) {
    let (mut next, mut changed) = (arr.clone(), false);
    let (n, m) = (arr.shape()[0] as i32, arr.shape()[1] as i32);
    for (x, y) in (0..n).cartesian_product(0..m) {
        let neighbors = vec![
            (x - 1, y),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y),
            (x + 1, y - 1),
            (x + 1, y + 1),
            (x, y - 1),
            (x, y + 1),
        ]
        .iter()
        .filter(|&(nx, ny)| {
            0 <= *nx && *nx < n && 0 <= *ny && *ny < m && arr[(*nx as usize, *ny as usize)] == '#'
        })
        .count();
        let mut state = arr[(x as usize, y as usize)];
        if state == 'L' && neighbors == 0 {
            state = '#';
            changed = true;
        } else if state == '#' && neighbors >= 4 {
            state = 'L';
            changed = true;
        }
        next[(x as usize, y as usize)] = state;
    }
    (next, changed)
}

fn iterate2(arr: &Array2<char>) -> (Array2<char>, bool) {
    let (mut next, mut changed) = (arr.clone(), false);
    let (n, m) = (arr.shape()[0] as i32, arr.shape()[1] as i32);
    for (x, y) in (0..n).cartesian_product(0..m) {
        let mut neighbors = 0;
        for (dx, dy) in (-1..=1).cartesian_product(-1..=1) {
            let (mut cx, mut cy) = (x + dx, y + dy);
            while !(dx == 0 && dy == 0) && cx >= 0 && cy >= 0 && cx < n && cy < m {
                match arr[(cx as usize, cy as usize)] {
                    '#' => {
                        neighbors += 1;
                        break;
                    }
                    'L' => {
                        break;
                    }
                    _ => {
                        cx += dx;
                        cy += dy;
                    }
                }
            }
        }
        let mut state = arr[(x as usize, y as usize)];
        if state == 'L' && neighbors == 0 {
            state = '#';
            changed = true;
        } else if state == '#' && neighbors >= 5 {
            state = 'L';
            changed = true;
        }
        next[(x as usize, y as usize)] = state;
    }
    (next, changed)
}
fn main() {
    let input = include_str!("11.txt")
        .split_terminator("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let chars = input
        .iter()
        .flat_map(|l| l.iter().cloned())
        .collect::<Vec<char>>();
    let arr = Array2::from_shape_vec((input.len(), input[0].len()), chars).unwrap();

    let (mut curr, mut changed) = (arr.clone().into_owned(), true);
    while changed {
        let res = iterate1(&curr.into_owned());
        curr = res.0;
        changed = res.1;
    }
    println!("Part 1: {}", curr.iter().filter(|c| **c == '#').count());

    let (mut curr, mut changed) = (arr.clone().into_owned(), true);
    while changed {
        let res = iterate2(&curr.into_owned());
        curr = res.0;
        changed = res.1;
    }
    println!("Part 2: {}", curr.iter().filter(|c| **c == '#').count());
}

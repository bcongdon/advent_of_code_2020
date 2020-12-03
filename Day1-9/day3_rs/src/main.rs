type Grid = Vec<Vec<char>>;
type Slope = (usize, usize);

fn toboggan(grid: &Grid, slope: Slope) -> usize {
    let (m, n) = (grid.len(), grid[0].len());
    let (mut x, mut y, mut count) = (0, 0, 0);
    while y < m {
        if grid[y][x % n] == '#' {
            count += 1;
        }
        x += slope.0;
        y += slope.1;
    }
    count
}

fn main() {
    let grid = include_str!("3.txt")
        .split_terminator("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("Part 1: {}", toboggan(&grid, (3, 1)));

    let part2: usize = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&slope| toboggan(&grid, slope))
        .product();
    println!("Part 2: {}", part2)
}

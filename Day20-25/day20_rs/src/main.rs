use std::collections::{HashMap, HashSet};

const N: usize = 10;
const PUZZLE_N: usize = 12;

const SEA_MONSTER: &str = "                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";
const SEA_MONSTER_SIZE: (usize, usize) = (20, 3);

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    grid: Vec<Vec<char>>,
    flipped: bool,
    rotation: usize,
}

enum Edge {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, PartialEq, Eq)]
enum TileClass {
    Corner,
    Edge,
    Interior,
}

impl Tile {
    fn edges(&self) -> Vec<String> {
        assert_eq!(self.grid.len(), N);
        assert_eq!(self.grid[0].len(), N);
        let mut edges: Vec<String> = vec![
            self.edge(Edge::Top),
            self.edge(Edge::Left),
            self.edge(Edge::Bottom),
            self.edge(Edge::Right),
        ];
        edges.append(&mut edges.iter().map(|s| s.chars().rev().collect()).collect());
        edges
    }

    fn transform_coord(&self, mut x: usize, mut y: usize) -> (usize, usize) {
        if self.flipped {
            x = N - x - 1
        };
        for _ in 0..self.rotation {
            std::mem::swap(&mut x, &mut y);
            x = N - x - 1
        }
        (x, y)
    }

    fn edge(&self, which: Edge) -> String {
        (0..N)
            .map(|idx| {
                let (x, y) = match which {
                    Edge::Top => self.transform_coord(idx, 0),
                    Edge::Left => self.transform_coord(0, idx),
                    Edge::Bottom => self.transform_coord(idx, N - 1),
                    Edge::Right => self.transform_coord(N - 1, idx),
                };
                self.grid[x][y]
            })
            .collect()
    }

    fn matches(&self, side: Edge, other: &Self, other_side: Edge) -> bool {
        self.edge(side) == other.edge(other_side)
    }

    fn classify(&self, edge_map: &HashMap<String, Vec<Tile>>) -> TileClass {
        let empty_edges = self
            .edges()
            .iter()
            .filter(|&e| edge_map[e].len() == 1)
            .count();
        match empty_edges {
            0 => TileClass::Interior,
            2 => TileClass::Edge,
            4 => TileClass::Corner,
            _ => panic!("Unknown tile type: {}", empty_edges),
        }
    }

    fn permutate(&mut self) {
        self.rotation += 1;
        if self.rotation == 4 {
            self.flipped = !self.flipped;
            self.rotation = 0;
        }
    }
}

fn parse_tile(raw: &str) -> Tile {
    let mut lines = raw.lines();
    let id = &lines.next().unwrap()[5..]
        .replace(":", "")
        .parse::<u64>()
        .unwrap();
    let grid = lines
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Tile {
        id: *id,
        grid,
        flipped: false,
        rotation: 0,
    }
}

type Mosaic = [[char; PUZZLE_N * (N - 2)]; PUZZLE_N * (N - 2)];

fn assemble_tile_grid(tiles: &Vec<Tile>) -> Mosaic {
    let mut edge_map: HashMap<String, Vec<Tile>> = HashMap::new();
    for t in tiles {
        for e in t.edges() {
            edge_map.entry(e).or_insert(vec![]).push(t.clone());
        }
    }

    let mut tile_grid: HashMap<(usize, usize), Tile> = HashMap::new();
    let mut first_corner = tiles
        .iter()
        .filter(|t| t.classify(&edge_map) == TileClass::Corner)
        .next()
        .unwrap()
        .clone();
    while !(edge_map.get(&first_corner.edge(Edge::Left)).unwrap().len() == 1
        && edge_map.get(&first_corner.edge(Edge::Top)).unwrap().len() == 1)
    {
        first_corner.permutate();
    }
    tile_grid.insert((0, 0), first_corner.clone());

    let mut used_tiles: HashSet<u64> = HashSet::new();
    used_tiles.insert(first_corner.id);

    // Fill in top
    for i in 1..PUZZLE_N {
        let prev = tile_grid.get(&(i - 1, 0)).unwrap();
        let mut other = edge_map
            .get(&prev.edge(Edge::Right))
            .unwrap()
            .iter()
            .filter(|t| !used_tiles.contains(&t.id))
            .next()
            .unwrap()
            .clone();
        assert_ne!(other.classify(&edge_map), TileClass::Interior);
        while !prev.matches(Edge::Right, &other, Edge::Left) {
            other.permutate();
        }
        used_tiles.insert(other.id);
        tile_grid.insert((i, 0), other);
    }
    // Fill in left-side
    for j in 1..PUZZLE_N {
        let prev = tile_grid.get(&(0, j - 1)).unwrap();
        let mut other = edge_map
            .get(&prev.edge(Edge::Bottom))
            .unwrap()
            .iter()
            .filter(|t| !used_tiles.contains(&t.id))
            .next()
            .unwrap()
            .clone();
        assert_ne!(other.classify(&edge_map), TileClass::Interior);
        while !prev.matches(Edge::Bottom, &other, Edge::Top) {
            other.permutate();
        }
        used_tiles.insert(other.id);
        tile_grid.insert((0, j), other);
    }

    // Fill in interior
    for i in 1..PUZZLE_N {
        for j in 1..PUZZLE_N {
            let prev_left = tile_grid.get(&(i - 1, j)).unwrap();
            let prev_up = tile_grid.get(&(i, j - 1)).unwrap();
            let mut other = edge_map
                .get(&prev_up.edge(Edge::Bottom))
                .unwrap()
                .iter()
                .filter(|t| !used_tiles.contains(&t.id))
                .next()
                .unwrap()
                .clone();
            while !prev_up.matches(Edge::Bottom, &other, Edge::Top)
                && !prev_left.matches(Edge::Right, &other, Edge::Left)
            {
                other.permutate();
            }
            used_tiles.insert(other.id);
            tile_grid.insert((i, j), other);
        }
    }

    let mut out = [[' '; PUZZLE_N * (N - 2)]; PUZZLE_N * (N - 2)];
    for ((x, y), t) in tile_grid {
        for i in 1..(N - 1) {
            for j in 1..(N - 1) {
                let (tx, ty) = t.transform_coord(i, j);
                out[x * (N - 2) + (i - 1)][y * (N - 2) + (j - 1)] = t.grid[tx][ty];
            }
        }
    }
    out
}

fn sea_monster_occurences(
    grid: &Mosaic,
    sea_monster_coords: &Vec<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut used = HashSet::new();
    for i in 0..grid.len() - SEA_MONSTER_SIZE.1 {
        for j in 0..grid[0].len() - SEA_MONSTER_SIZE.0 {
            let coords = sea_monster_coords
                .iter()
                .map(|&(x, y)| (x + i, y + j))
                .collect::<Vec<_>>();
            if coords.iter().all(|&(cx, cy)| grid[cx][cy] == '#') {
                coords.iter().for_each(|c| {
                    used.insert(c.clone());
                });
            }
        }
    }
    used
}

fn flip(grid: &Mosaic) -> Mosaic {
    let mut out = [[' '; PUZZLE_N * (N - 2)]; PUZZLE_N * (N - 2)];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            out[out.len() - 1 - i][j] = grid[i][j];
        }
    }
    out
}

fn rotate(grid: &Mosaic) -> Mosaic {
    let mut out = [[' '; PUZZLE_N * (N - 2)]; PUZZLE_N * (N - 2)];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            out[i][j] = grid[j][i];
        }
    }
    out
}

fn main() {
    let tiles = include_str!("20.txt")
        .split_terminator("\n\n")
        .map(parse_tile)
        .collect::<Vec<_>>();

    let mut edges_count: HashMap<String, usize> = HashMap::new();
    for e in tiles.iter().flat_map(|t| t.edges()) {
        *edges_count.entry(e).or_insert(0) += 1;
    }

    let part1: u64 = tiles
        .iter()
        .filter(|&t| t.edges().iter().filter(|&e| edges_count[e] == 1).count() == 4)
        .map(|t| t.id)
        .product();
    println!("Part 1: {}", part1);

    let mut mosaic = assemble_tile_grid(&tiles);
    let sea_monster_coords = SEA_MONSTER
        .lines()
        .enumerate()
        .flat_map(|(x, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(y, c)| if c == '#' { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    'outer: loop {
        for _ in 0..4 {
            let occurence_coords = sea_monster_occurences(&mosaic, &sea_monster_coords);
            if occurence_coords.len() > 0 {
                let mut no_monster_hashes = 0;
                for i in 0..mosaic.len() {
                    for j in 0..mosaic[0].len() {
                        if mosaic[i][j] == '#' && !occurence_coords.contains(&(i, j)) {
                            no_monster_hashes += 1;
                        }
                    }
                }
                println!("Part 2: {}", no_monster_hashes);
                break 'outer;
            }
            mosaic = rotate(&mosaic);
        }
        mosaic = flip(&mosaic);
    }
}

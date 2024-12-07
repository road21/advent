use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs, usize,
};

use itertools::Itertools;
use std::borrow::BorrowMut;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Visited { d: Direction },
    Start(Direction),
}

fn parse(contents: &str) -> Vec<Vec<Tile>> {
    let parts: Vec<&str> = contents.split("\n").into_iter().collect();

    parts
        .into_iter()
        .map(|i| {
            i.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'v' => Tile::Start(Direction::Down),
                    '^' => Tile::Start(Direction::Up),
                    '<' => Tile::Start(Direction::Left),
                    '>' => Tile::Start(Direction::Right),
                    _ => panic!("illegal format"),
                })
                .collect()
        })
        .collect()
}

fn has_cycle((i, j): (usize, usize), dir: Direction, mut tiles: Vec<Vec<Tile>>) -> bool {
    let next = match dir {
        Direction::Up if i > 0 => Some((i - 1, j)),
        Direction::Right if j < tiles[i].len() - 1 => Some((i, j + 1)),
        Direction::Down if i < tiles.len() - 1 => Some((i + 1, j)),
        Direction::Left if j > 0 => Some((i, j - 1)),
        _ => None,
    };

    match tiles[i][j] {
        Tile::Visited { d } => {}
        _ => tiles[i][j] = Tile::Visited { d: dir },
    };

    match next {
        Some((n_i, n_j)) => match tiles[n_i][n_j] {
            Tile::Visited { d } if dir == d => true,
            Tile::Visited { d } => has_cycle((n_i, n_j), dir, tiles),
            Tile::Empty => has_cycle((n_i, n_j), dir, tiles),
            Tile::Wall => {
                let new_dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
                has_cycle((i, j), new_dir, tiles)
            }
            Tile::Start(_) => panic!("wtf"),
        },
        None => false,
    }
}

pub fn day6_2() {
    let file_path = "6_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let tiles: Vec<Vec<Tile>> = parse(&contents);

    let start = tiles
        .iter()
        .enumerate()
        .find_map(|(n, raw)| {
            (*raw).iter().enumerate().find_map(|(j, t)| match t {
                Tile::Start(d) => Some((n, j, *d)),
                _ => None,
            })
        })
        .unwrap();

    let mut count = 0;
    for (i, row) in tiles.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            if (*x == Tile::Empty) {
                let mut new_tiles = tiles.clone();
                new_tiles[i][j] = Tile::Wall;
                if has_cycle((start.0, start.1), start.2, new_tiles) {
                    count = count + 1;
                }
            }
        }
    }

    println!("{:?}", count);
    // let res = run((start.0, start.1), start.2, tiles);
    // println!("{:?}", res)
}

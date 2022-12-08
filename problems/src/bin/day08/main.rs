use std::collections::HashSet;
use std::ops::{Add, Index};

const INPUT: &[u8] = include_bytes!("input.txt");
#[cfg(test)]
const EXAMPLE: &[u8] = include_bytes!("example.txt");

const UP: Coordinate = Coordinate { x: 0, y: -1 };
const RIGHT: Coordinate = Coordinate { x: 1, y: 0 };
const DOWN: Coordinate = Coordinate { x: 0, y: 1 };
const LEFT: Coordinate = Coordinate { x: -1, y: 0 };

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance(&self, other: &Coordinate) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Map {
    width: i32,
    height: i32,
    map: Vec<i8>,
}

impl Map {
    fn load(input: &[u8]) -> Self {
        let columns = input.iter().position(|c| *c == b'\n').unwrap();
        let map = input
            .iter()
            .filter_map(|c| (*c != b'\n').then(|| (c - b'0') as i8))
            .collect::<Vec<_>>();
        let rows = map.len() / columns;

        Self {
            width: columns as i32,
            height: rows as i32,
            map,
        }
    }

    fn top_left(&self) -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }

    fn top_right(&self) -> Coordinate {
        Coordinate {
            x: self.width - 1,
            y: 0,
        }
    }

    fn bottom_left(&self) -> Coordinate {
        Coordinate {
            x: 0,
            y: self.height - 1,
        }
    }

    fn map_index(&self, coordinate: Coordinate) -> usize {
        debug_assert!(self.in_bounds(coordinate));
        (coordinate.x * self.width + coordinate.y) as usize
    }

    fn in_bounds(&self, Coordinate { x, y }: Coordinate) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn iterate(&self, from: Coordinate, step: Coordinate) -> impl Iterator<Item = Coordinate> + '_ {
        let mut here = from;
        std::iter::from_fn(move || {
            if self.in_bounds(here) {
                let next = here + step;
                Some(std::mem::replace(&mut here, next))
            } else {
                None
            }
        })
    }

    fn all_points(&self) -> impl Iterator<Item = Coordinate> + '_ {
        self.iterate(self.top_left(), RIGHT)
            .flat_map(|c| self.iterate(c, DOWN))
    }
}

impl Index<Coordinate> for Map {
    type Output = i8;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        &self.map[self.map_index(coord)]
    }
}

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn visible_trees<'a>(
    map: &'a Map,
    sight_line: impl Iterator<Item = Coordinate> + 'a,
) -> impl Iterator<Item = Coordinate> + 'a {
    let mut highest_tree_height = -1;
    sight_line.filter(move |xy| {
        let tree_height = map[*xy];
        if tree_height > highest_tree_height {
            highest_tree_height = tree_height;
            true
        } else {
            false
        }
    })
}

fn part1(input: &[u8]) -> usize {
    let map = Map::load(input);

    let right = map
        .iterate(map.top_left(), DOWN)
        .flat_map(|c| visible_trees(&map, map.iterate(c, RIGHT)));
    let left = map
        .iterate(map.top_right(), DOWN)
        .flat_map(|c| visible_trees(&map, map.iterate(c, LEFT)));
    let down = map
        .iterate(map.top_left(), RIGHT)
        .flat_map(|c| visible_trees(&map, map.iterate(c, DOWN)));
    let up = map
        .iterate(map.bottom_left(), RIGHT)
        .flat_map(|c| visible_trees(&map, map.iterate(c, UP)));

    right
        .chain(left)
        .chain(down)
        .chain(up)
        .collect::<HashSet<_>>()
        .len()
}

#[test]
fn part1_example() {
    assert_eq!(21, part1(EXAMPLE))
}

#[test]
#[ignore]
fn part1_verify() {
    assert_eq!(1792, part1(INPUT))
}

fn score_direction(map: &Map, coord: Coordinate, step: Coordinate) -> Option<i32> {
    let height = map[coord];
    map.iterate(coord + step, step)
        .find(|c| map[*c] >= height)
        .map(|c| coord.manhattan_distance(&c))
}

fn score(map: &Map, coord: Coordinate) -> i32 {
    let up = score_direction(map, coord, UP).unwrap_or(coord.y);
    let right = score_direction(map, coord, RIGHT).unwrap_or(map.width - coord.x - 1);
    let down = score_direction(map, coord, DOWN).unwrap_or(map.height - coord.y - 1);
    let left = score_direction(map, coord, LEFT).unwrap_or(coord.x);
    up * right * down * left
}

fn part2(input: &[u8]) -> i32 {
    let map = Map::load(input);

    map.all_points()
        .map(|coord| score(&map, coord))
        .max()
        .unwrap()
}

#[test]
fn part2_example() {
    assert_eq!(8, part2(EXAMPLE))
}

#[test]
#[ignore]
fn part2_verify() {
    assert_eq!(334880, part2(INPUT))
}

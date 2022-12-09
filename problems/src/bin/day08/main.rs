use adventofcode2022_common::vec2i::{Bounds, Point, Vector, DOWN, LEFT, RIGHT, UP};
use std::collections::HashSet;
use std::ops::Index;

const INPUT: &[u8] = include_bytes!("input.txt");
#[cfg(test)]
const EXAMPLE: &[u8] = include_bytes!("example.txt");

struct Map {
    bounds: Bounds,
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
            bounds: Bounds::new(0, (columns - 1) as i32, (rows - 1) as i32, 0),
            map,
        }
    }

    fn iterate(&self, from: Point, step: Vector) -> impl Iterator<Item = Point> + '_ {
        let mut here = from;
        std::iter::from_fn(move || {
            if self.bounds.contains(&here) {
                let next = here + step;
                Some(std::mem::replace(&mut here, next))
            } else {
                None
            }
        })
    }

    fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.iterate(self.bounds.top_left(), RIGHT)
            .flat_map(|c| self.iterate(c, DOWN))
    }
}

impl Index<Point> for Map {
    type Output = i8;

    fn index(&self, coord: Point) -> &Self::Output {
        &self.map[self.bounds.index(&coord)]
    }
}

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn visible_trees<'a>(
    map: &'a Map,
    sight_line: impl Iterator<Item = Point> + 'a,
) -> impl Iterator<Item = Point> + 'a {
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
        .iterate(map.bounds.top_left(), DOWN)
        .flat_map(|c| visible_trees(&map, map.iterate(c, RIGHT)));
    let left = map
        .iterate(map.bounds.top_right(), DOWN)
        .flat_map(|c| visible_trees(&map, map.iterate(c, LEFT)));
    let down = map
        .iterate(map.bounds.top_left(), RIGHT)
        .flat_map(|c| visible_trees(&map, map.iterate(c, DOWN)));
    let up = map
        .iterate(map.bounds.bottom_left(), RIGHT)
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

fn score_direction(map: &Map, coord: Point, step: Vector) -> Option<i32> {
    let height = map[coord];
    map.iterate(coord + step, step)
        .find(|c| map[*c] >= height)
        .map(|c| coord.manhattan_distance(&c))
}

fn score(map: &Map, coord: Point) -> i32 {
    let size = map.bounds.size();
    let up = score_direction(map, coord, UP).unwrap_or(coord.y());
    let right = score_direction(map, coord, RIGHT).unwrap_or(size.width() - coord.x() - 1);
    let down = score_direction(map, coord, DOWN).unwrap_or(size.height() - coord.y() - 1);
    let left = score_direction(map, coord, LEFT).unwrap_or(coord.x());
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

use adventofcode2022_common::grid2d::Grid2d;
use adventofcode2022_common::vec2i::{Point, Vector, DOWN, LEFT, RIGHT, UP};
use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("input.txt");
#[cfg(test)]
const EXAMPLE: &[u8] = include_bytes!("example.txt");

type Map = Grid2d<i8>;

fn load(input: &[u8]) -> Map {
    let columns = input.iter().position(|c| *c == b'\n').unwrap();
    let tiles = input
        .iter()
        .filter_map(|c| (*c != b'\n').then(|| (c - b'0') as i8))
        .collect::<Vec<_>>();

    Grid2d::from_parts(0, columns as i32, tiles)
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
    let map = load(input);

    let bounds = map.bounds;
    let right = bounds
        .walk(bounds.top_left(), DOWN)
        .flat_map(|c| visible_trees(&map, bounds.walk(c, RIGHT)));
    let left = bounds
        .walk(bounds.top_right(), DOWN)
        .flat_map(|c| visible_trees(&map, bounds.walk(c, LEFT)));
    let down = bounds
        .walk(bounds.top_left(), RIGHT)
        .flat_map(|c| visible_trees(&map, bounds.walk(c, DOWN)));
    let up = bounds
        .walk(bounds.bottom_left(), RIGHT)
        .flat_map(|c| visible_trees(&map, bounds.walk(c, UP)));

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
    let bounds = map.bounds;
    let score = bounds
        .walk(coord + step, step)
        .find(|c| map[*c] >= height)
        .map(|c| coord.manhattan_distance(&c));
    score
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
    let map = load(input);

    map.bounds
        .iter_points()
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

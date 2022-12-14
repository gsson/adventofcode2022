use adventofcode2022_common::grid2d::Grid2d;
use adventofcode2022_common::vec2i::{Bounds, Point};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

type HeightMap = Grid2d<i8>;
type CostMap = Grid2d<i32>;

fn parse_input(input: &str) -> (Point, Point, HeightMap) {
    let columns = input.chars().position(|c| c == '\n').unwrap() as i32;
    let start = input
        .chars()
        .filter(|c| *c != '\n')
        .position(|c| c == 'S')
        .unwrap();
    let goal = input
        .chars()
        .filter(|c| *c != '\n')
        .position(|c| c == 'E')
        .unwrap();
    let heights = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            'S' => 0,
            'E' => 25,
            'a'..='z' => c as i8 - 'a' as i8,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let rows = heights.len() as i32 / columns;
    let heights = HeightMap::from_parts(i8::MAX, Bounds::with_size([columns, rows]), heights);
    let start = heights.from_index(start);
    let goal = heights.from_index(goal);
    (start, goal, heights)
}

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn find_best_trail(starts: &[Point], goal: Point, heights: &HeightMap) -> i32 {
    let bounds = heights.bounds;
    let mut costs = CostMap::with_bounds(i32::MAX, bounds);
    starts.iter().for_each(|start| costs[*start] = 0);

    let mut next =
        BinaryHeap::from_iter(starts.iter().map(|start| (Reverse(costs[*start]), *start)));

    while let Some((_, current)) = next.pop() {
        let max_next_height = heights[current] + 1;
        let next_cost = costs[current] + 1;
        for p in bounds.cardinals(current) {
            if heights[p] <= max_next_height && costs[p] > next_cost {
                costs[p] = next_cost;
                if p == goal {
                    break;
                }

                next.push((Reverse(next_cost), p));
            }
        }
    }
    costs[goal]
}

fn part1(input: &str) -> i32 {
    let (start, goal, heights) = parse_input(input);
    find_best_trail(&[start], goal, &heights)
}

fn part2(input: &str) -> i32 {
    let (_, goal, heights) = parse_input(input);
    let starts = heights
        .bounds
        .iter_points()
        .filter(|p| heights[*p] == 0)
        .collect::<Vec<_>>();
    find_best_trail(&starts, goal, &heights)
}

#[test]
fn part1_example() {
    assert_eq!(31, part1(EXAMPLE))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(391, part1(INPUT))
}

#[test]
fn part2_example() {
    assert_eq!(29, part2(EXAMPLE))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(386, part2(INPUT))
}

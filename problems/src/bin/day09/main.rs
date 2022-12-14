use adventofcode2022_common::vec2i::{Point, Vector, DOWN, LEFT, RIGHT, UP};
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE1: &str = include_str!("example1.txt");
#[cfg(test)]
const EXAMPLE2: &str = include_str!("example2.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn parse_move(line: &str) -> impl Iterator<Item = Vector> {
    let (direction, length) = line.split_once(' ').unwrap();
    let direction = match direction {
        "U" => UP,
        "R" => RIGHT,
        "D" => DOWN,
        "L" => LEFT,
        _ => unreachable!(),
    };
    let length = length.parse::<usize>().unwrap();
    std::iter::repeat(direction).take(length)
}

fn parse_moves(input: &str) -> impl Iterator<Item = Vector> + '_ {
    input.lines().flat_map(parse_move)
}

fn move_tail(head: &Point, tail: &Point) -> Point {
    let diff = head.vector(tail);
    if diff.abs().max_component() <= 1 {
        *tail
    } else {
        *tail + diff.signum()
    }
}

fn part1(input: &str) -> usize {
    let tail_positions = parse_moves(input)
        .scan(Point::default(), |head, mv| {
            *head += mv;
            Some(*head)
        })
        .scan(Point::default(), |tail, head| {
            *tail = move_tail(&head, tail);
            Some(*tail)
        })
        .collect::<HashSet<_>>();
    tail_positions.len()
}

#[test]
fn part1_example() {
    assert_eq!(13, part1(EXAMPLE1))
}

#[test]
#[ignore]
fn part1_verify() {
    assert_eq!(5883, part1(INPUT))
}

fn part2(input: &str) -> usize {
    let tail_positions = parse_moves(input)
        .scan(
            (Point::default(), [Point::default(); 9]),
            |(head, tails), mv| {
                *head += mv;
                let end = tails.iter_mut().fold(*head, |prev, current| {
                    *current = move_tail(&prev, current);
                    *current
                });
                Some(end)
            },
        )
        .collect::<HashSet<_>>();

    tail_positions.len()
}

#[test]
fn part2_example() {
    assert_eq!(1, part2(EXAMPLE1));
    assert_eq!(36, part2(EXAMPLE2))
}

#[test]
#[ignore]
fn part2_verify() {
    assert_eq!(2367, part2(INPUT))
}

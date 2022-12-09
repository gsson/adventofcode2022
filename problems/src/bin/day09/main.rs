use std::cmp::max;
use std::collections::HashSet;
use std::ops::{Add, Sub};

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE1: &str = include_str!("example1.txt");
#[cfg(test)]
const EXAMPLE2: &str = include_str!("example2.txt");

const UP: Coordinate = Coordinate { x: 0, y: -1 };
const RIGHT: Coordinate = Coordinate { x: 1, y: 0 };
const DOWN: Coordinate = Coordinate { x: 0, y: 1 };
const LEFT: Coordinate = Coordinate { x: -1, y: 0 };

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
    fn max_component(&self) -> i32 {
        max(self.x, self.y)
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

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn parse_move(line: &str) -> impl Iterator<Item = Coordinate> {
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

fn parse_moves(input: &str) -> impl Iterator<Item = Coordinate> + '_ {
    input.lines().flat_map(parse_move)
}

fn move_tail(head: &Coordinate, tail: &Coordinate) -> Coordinate {
    let diff = *head - *tail;
    if diff.abs().max_component() <= 1 {
        *tail
    } else {
        *tail + diff.signum()
    }
}

fn part1(input: &str) -> usize {
    let tail_positions = parse_moves(input)
        .scan(Coordinate::default(), |head, m| {
            *head = *head + m;
            Some(*head)
        })
        .scan(Coordinate::default(), |tail, head| {
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
            (Coordinate::default(), [Coordinate::default(); 9]),
            |(head, tails), mv| {
                *head = *head + mv;
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

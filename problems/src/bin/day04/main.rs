use std::cmp::{max, min};
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

struct RangeInclusive(i32, i32);

impl FromStr for RangeInclusive {
    type Err = ParseIntError;

    fn from_str(from_to: &str) -> Result<Self, Self::Err> {
        let (from, to) = from_to.split_once('-').unwrap();
        let from = from.parse::<i32>()?;
        let to = to.parse::<i32>()?;
        Ok(Self(from, to))
    }
}

impl RangeInclusive {
    fn length(&self) -> i32 {
        1 + self.1 - self.0
    }

    fn intersect(&self, other: &RangeInclusive) -> Option<RangeInclusive> {
        let from = max(self.0, other.0);
        let to = min(self.1, other.1);
        (from <= to).then_some(RangeInclusive(from, to))
    }

    fn fully_overlaps(&self, other: &RangeInclusive) -> bool {
        if let Some(intersect) = self.intersect(other) {
            intersect.length() == min(self.length(), other.length())
        } else {
            false
        }
    }

    fn overlaps(&self, other: &RangeInclusive) -> bool {
        self.intersect(other).is_some()
    }
}

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn read_elf_pair(elf_pair: &str) -> (RangeInclusive, RangeInclusive) {
    let (first, second) = elf_pair.split_once(',').unwrap();
    let first = first.parse().unwrap();
    let second = second.parse().unwrap();
    (first, second)
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(read_elf_pair)
        .filter(|(a, b)| a.fully_overlaps(b))
        .count() as i32
}

#[test]
fn part1_example() {
    assert_eq!(2, part1(EXAMPLE))
}

#[test]
#[ignore]
fn part1_verify() {
    assert_eq!(518, part1(INPUT))
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(read_elf_pair)
        .filter(|(a, b)| a.overlaps(b))
        .count() as i32
}

#[test]
fn part2_example() {
    assert_eq!(4, part2(EXAMPLE))
}

#[test]
#[ignore]
fn part2_verify() {
    assert_eq!(909, part2(INPUT))
}

#![feature(iter_array_chunks)]

use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn item_priority(c: char) -> i32 {
    match c {
        'a'..='z' => (c as i32 - 'a' as i32) + 1,
        'A'..='Z' => (c as i32 - 'A' as i32) + 27,
        _ => unreachable!(),
    }
}

fn backpack_priority(contents: &str) -> i32 {
    let (first, second) = contents.split_at(contents.len() / 2);
    let second = second.chars().collect::<HashSet<_>>();
    first
        .chars()
        .filter(|c| second.contains(c))
        .map(item_priority)
        .next()
        .expect("a thing")
}

fn part1(input: &str) -> i32 {
    input.lines().map(backpack_priority).sum()
}

#[test]
fn part1_example() {
    assert_eq!(157, part1(EXAMPLE))
}

#[test]
#[ignore]
fn part1_verify() {
    assert_eq!(7727, part1(INPUT))
}

fn group_priority([first, second, third]: [&str; 3]) -> i32 {
    let second = second.chars().collect::<HashSet<_>>();
    let third = third.chars().collect::<HashSet<_>>();

    first
        .chars()
        .filter(|c| second.contains(c))
        .filter(|c| third.contains(c))
        .map(item_priority)
        .next()
        .expect("a thing")
}

fn part2(input: &str) -> i32 {
    input.lines().array_chunks::<3>().map(group_priority).sum()
}

#[test]
fn part2_example() {
    assert_eq!(70, part2(EXAMPLE))
}

#[test]
#[ignore]
fn part2_verify() {
    assert_eq!(2609, part2(INPUT))
}

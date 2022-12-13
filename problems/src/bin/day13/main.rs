use crate::Value::{List, Number};
use std::cmp::Ordering;
use std::slice;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn parse_value_pairs(input: &str) -> impl Iterator<Item = (Value, Value)> + '_ {
    input
        .split_terminator("\n\n")
        .filter_map(|pair| pair.split_once('\n'))
        .map(|(first, second)| (first.parse().unwrap(), second.parse().unwrap()))
}

fn parse_values(input: &str) -> impl Iterator<Item = Value> + '_ {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
}

#[derive(Clone, Debug)]
enum Value {
    List(Vec<Value>),
    Number(i32),
}

fn consume_number(number: &str) -> (Value, &str) {
    if let Some(i) = number.find(|c: char| !c.is_ascii_digit()) {
        let (number, tail) = number.split_at(i);
        (Number(number.parse().unwrap()), tail)
    } else {
        (Number(number.parse().unwrap()), "")
    }
}

fn consume_list(mut list: &str) -> (Value, &str) {
    list = list.strip_prefix('[').unwrap();

    let mut values = Vec::new();
    while !list.starts_with(']') {
        let (value, tail) = consume_value(list);
        values.push(value);
        list = tail.strip_prefix(',').unwrap_or(tail);
    }
    let tail = list.strip_prefix(']').unwrap();

    (List(values), tail)
}

fn consume_value(value: &str) -> (Value, &str) {
    if value.starts_with('[') {
        consume_list(value)
    } else if value.starts_with(|c: char| c.is_ascii_digit()) {
        consume_number(value)
    } else {
        unreachable!()
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(consume_value(s).0)
    }
}

impl Eq for Value {}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        fn compare_slices(left: &[Value], right: &[Value]) -> Ordering {
            match (left, right) {
                ([], []) => Ordering::Equal,
                ([], _) => Ordering::Less,
                (_, []) => Ordering::Greater,
                ([h1, t1 @ ..], [h2, t2 @ ..]) => h1.cmp(h2).then_with(|| compare_slices(t1, t2)),
            }
        }
        match (self, other) {
            (Number(left), Number(right)) => left.cmp(right),
            (left @ Number(_), List(right)) => compare_slices(slice::from_ref(left), right),
            (List(left), right @ Number(_)) => compare_slices(left, slice::from_ref(right)),
            (List(left), List(right)) => compare_slices(left, right),
        }
    }
}

fn part1(input: &str) -> usize {
    parse_value_pairs(input)
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .fold(0, |a, (i, _)| a + i + 1)
}

fn part2(input: &str) -> usize {
    let divider1 = List(vec![List(vec![Number(2)])]);
    let divider2 = List(vec![List(vec![Number(6)])]);
    let mut v = parse_values(input).collect::<Vec<_>>();
    v.sort_unstable();
    let a = v.binary_search(&divider1).unwrap_err() + 1;
    let b = v.binary_search(&divider2).unwrap_err() + 2; // Add one for the index of divider1 that wasn't actually inserted
    a * b
}

#[test]
fn part1_example() {
    assert_eq!(13, part1(EXAMPLE))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(5292, part1(INPUT))
}

#[test]
fn part2_example() {
    assert_eq!(140, part2(EXAMPLE))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(23868, part2(INPUT))
}

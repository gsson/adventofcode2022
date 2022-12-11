use crate::Op::{AddBy, MulBy, Square};
use std::cmp::Reverse;
use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

type WorryLevel = i64;

fn words(line: &str) -> Vec<&str> {
    line.split_ascii_whitespace().collect()
}

fn entry(line: &str) -> &str {
    let (_, value) = line.split_once(": ").expect("value");
    value
}

enum Op {
    AddBy(WorryLevel),
    MulBy(WorryLevel),
    Square,
}

impl Op {
    fn apply(&self, item: WorryLevel) -> WorryLevel {
        match self {
            AddBy(i) => item + i,
            MulBy(i) => item * i,
            Square => item * item,
        }
    }
}

fn parse_monkey(monkey: &str) -> Monkey {
    let lines = monkey.lines().collect::<Vec<_>>();
    let items = entry(lines[1])
        .split_terminator(", ")
        .map(|item| item.parse::<WorryLevel>().expect("item number"))
        .collect::<VecDeque<_>>();

    let operation = words(entry(lines[2]));
    let operation = match operation.as_slice() {
        ["new", "=", "old", "+", value] => AddBy(value.parse().unwrap()),
        ["new", "=", "old", "*", "old"] => Square,
        ["new", "=", "old", "*", value] => MulBy(value.parse().unwrap()),
        _ => unreachable!(),
    };
    let (_, is_divisible_by) = entry(lines[3]).rsplit_once(' ').unwrap();
    let is_divisible_by = is_divisible_by.parse::<WorryLevel>().unwrap();
    let (_, if_true) = lines[4].rsplit_once(' ').unwrap();
    let if_true = if_true.parse::<usize>().unwrap();
    let (_, if_false) = lines[5].rsplit_once(' ').unwrap();
    let if_false = if_false.parse::<usize>().unwrap();
    Monkey {
        items,
        operation,
        is_divisible_by,
        if_true,
        if_false,
    }
}

struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Op,
    is_divisible_by: WorryLevel,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn throw<F: Fn(WorryLevel) -> WorryLevel>(
        &mut self,
        worry_level_modifier: F,
    ) -> Option<(usize, WorryLevel)> {
        if let Some(item) = self.items.pop_front() {
            let worry_level = worry_level_modifier(self.operation.apply(item));
            let target_monkey = if worry_level % self.is_divisible_by == 0 {
                self.if_true
            } else {
                self.if_false
            };
            Some((target_monkey, worry_level))
        } else {
            None
        }
    }
    fn receive(&mut self, item: WorryLevel) {
        self.items.push_back(item);
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split_terminator("\n\n").map(parse_monkey).collect()
}

fn monkey_business<const N: usize, F: Fn(WorryLevel) -> WorryLevel>(
    mut monkeys: Vec<Monkey>,
    worry_level_modifier: F,
) -> i64 {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..N {
        for monkey_number in 0..monkeys.len() {
            while let Some((target, item)) = monkeys[monkey_number].throw(&worry_level_modifier) {
                inspections[monkey_number] += 1;
                monkeys[target].receive(item)
            }
        }
    }

    inspections.sort_unstable_by_key(|s| Reverse(*s));
    inspections.into_iter().take(2).product()
}

fn part1(input: &str) -> i64 {
    let monkeys = parse_input(input);
    monkey_business::<20, _>(monkeys, |w| w / 3)
}

#[test]
fn part1_example() {
    assert_eq!(10605, part1(EXAMPLE))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(120384, part1(INPUT))
}

fn part2(input: &str) -> i64 {
    let monkeys = parse_input(input);
    let worry_modifier = monkeys
        .iter()
        .map(|i| i.is_divisible_by)
        .product::<WorryLevel>();
    monkey_business::<10_000, _>(monkeys, |w| w % worry_modifier)
}

#[test]
fn part2_example() {
    assert_eq!(2713310158, part2(EXAMPLE))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(32059801242, part2(INPUT))
}

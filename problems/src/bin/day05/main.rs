use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

const COLUMN_WIDTH: usize = 4;

fn parse_stacks(initial: &str) -> Vec<VecDeque<char>> {
    fn box_label(row: &str, index: usize) -> Option<char> {
        row.chars()
            .nth(index * COLUMN_WIDTH + 1)
            .filter(|c| c.is_ascii_alphabetic())
    }

    let (stacks, column_labels) = initial.rsplit_once('\n').unwrap();

    let column_count = (column_labels.len() + 3) / COLUMN_WIDTH;
    let mut columns: Vec<VecDeque<char>> = vec![VecDeque::new(); column_count];
    for row in stacks.lines() {
        (0..column_count)
            .filter_map(|column_index| {
                box_label(row, column_index).map(|label| (column_index, label))
            })
            .for_each(|(i, label)| columns[i].push_front(label));
    }

    columns
}

fn parse_instructions(instructions: &str) -> Vec<(usize, usize, usize)> {
    fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
        let s = instruction.split_terminator(' ').collect::<Vec<_>>();
        match s.as_slice() {
            ["move", count, "from", from, "to", to] => (
                count.parse::<usize>().unwrap(),
                from.parse::<usize>().unwrap() - 1,
                to.parse::<usize>().unwrap() - 1,
            ),
            _ => unreachable!(),
        }
    }

    instructions.lines().map(parse_instruction).collect()
}

fn pick_stacks(
    stacks: &mut [VecDeque<char>],
    from: usize,
    to: usize,
) -> (&mut VecDeque<char>, &mut VecDeque<char>) {
    assert_ne!(from, to);
    let first = from.min(to);
    let second = from.max(to);
    if let [first, .., second] = &mut stacks[first..=second] {
        if from < to {
            (first, second)
        } else {
            (second, first)
        }
    } else {
        unreachable!()
    }
}

fn part1(input: &str) -> String {
    let (initial, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(initial);

    let instructions = parse_instructions(instructions);

    for (count, from, to) in instructions {
        let from_index = stacks[from].len() - count;
        let (from, to) = pick_stacks(&mut stacks, from, to);
        to.extend(from.drain(from_index..).rev());
    }

    stacks.iter().map(|s| s.back().unwrap()).collect::<String>()
}

#[test]
fn part1_example() {
    assert_eq!("CMZ", part1(EXAMPLE))
}

#[test]
#[ignore]
fn part1_verify() {
    assert_eq!("VWLCWGSDQ", part1(INPUT))
}

fn part2(input: &str) -> String {
    let (initial, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(initial);

    let instructions = parse_instructions(instructions);

    for (count, from, to) in instructions {
        let from_index = stacks[from].len() - count;
        let (from, to) = pick_stacks(&mut stacks, from, to);
        to.extend(from.drain(from_index..));
    }

    stacks.iter().map(|s| s.back().unwrap()).collect::<String>()
}

#[test]
fn part2_example() {
    assert_eq!("MCD", part2(EXAMPLE))
}

#[test]
#[ignore]
fn part2_verify() {
    assert_eq!("TCGLQSLPW", part2(INPUT))
}

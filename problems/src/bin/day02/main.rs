use adventofcode2022_common::iter::UnwrapSingle;

const INPUT: &str = include_str!("input.txt");

const LOSE: i32 = 0;
const DRAW: i32 = 1;
const WIN: i32 = 2;

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn score(their_move: i32, your_move: i32) -> i32 {
    let outcome = (your_move - their_move + 1 + 3) % 3;

    your_move + 1 + outcome * 3
}

fn columns(line: &str) -> (i32, i32) {
    let (abc, xyz) = line.split_once(' ').expect("the inquisition");
    let (abc, xyz) = (
        abc.chars().unwrap_single() as i32 - 'A' as i32,
        xyz.chars().unwrap_single() as i32 - 'X' as i32,
    );
    (abc, xyz)
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(columns)
        .map(|(their_move, your_move)| score(their_move, your_move))
        .sum()
}

#[test]
fn part1_example() {
    assert_eq!(15, part1("A Y\nB X\nC Z"))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(15572, part1(INPUT))
}

fn winning_move_against(their_move: i32) -> i32 {
    (their_move + 1) % 3
}

fn losing_move_against(their_move: i32) -> i32 {
    (their_move + 2) % 3
}

fn drawing_move_against(their_move: i32) -> i32 {
    their_move
}

fn determine_move(their_move: i32, your_strategy: i32) -> (i32, i32) {
    let your_move = match your_strategy {
        LOSE => losing_move_against(their_move),
        DRAW => drawing_move_against(their_move),
        WIN => winning_move_against(their_move),
        _ => unreachable!(),
    };

    (their_move, your_move)
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(columns)
        .map(|(their_move, your_strategy)| determine_move(their_move, your_strategy))
        .map(|(their_move, your_move)| score(their_move, your_move))
        .sum()
}

#[test]
fn part2_example() {
    assert_eq!(12, part2("A Y\nB X\nC Z"))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(16098, part2(INPUT))
}

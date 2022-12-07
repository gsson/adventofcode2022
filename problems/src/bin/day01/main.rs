const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn sum_lines(lines: &str) -> i32 {
    lines
        .lines()
        .map(|line| line.parse::<i32>().expect("number"))
        .sum()
}

fn part1(input: &str) -> i32 {
    input
        .split_terminator("\n\n")
        .map(sum_lines)
        .max()
        .expect("an elf")
}

#[test]
fn part1_example() {
    assert_eq!(24000, part1(EXAMPLE))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(74394, part1(INPUT))
}

const fn top3(arr: [i32; 3], v: i32) -> [i32; 3] {
    match arr {
        [_, _, c] if c > v => arr,
        [a, b, _] if b > v => [a, b, v],
        [a, b, _] if a > v => [a, v, b],
        [a, b, _] => [v, a, b],
    }
}

fn part2(input: &str) -> i32 {
    let [a, b, c] = input
        .split_terminator("\n\n")
        .map(sum_lines)
        .fold([0, 0, 0], top3);

    a + b + c
}

#[test]
fn part2_example() {
    assert_eq!(45000, part2(EXAMPLE))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(212836, part2(INPUT))
}

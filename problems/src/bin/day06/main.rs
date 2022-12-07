#![feature(array_windows)]

const INPUT: &[u8] = include_bytes!("input.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn unique_bytes<const N: usize>(window: &[u8; N]) -> bool {
    window
        .iter()
        .fold(0u128, |set, byte| set | 1u128 << byte)
        .count_ones() as usize
        == N
}

fn start_of_signal<const N: usize>(input: &[u8]) -> usize {
    input.array_windows::<N>().position(unique_bytes).unwrap() + N
}

fn part1(input: &[u8]) -> usize {
    start_of_signal::<4>(input)
}

#[test]
fn part1_example() {
    assert_eq!(7, part1(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, part1(b"bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, part1(b"nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, part1(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, part1(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}

#[test]
#[ignore]
fn part1_verify() {
    assert_eq!(1833, part1(INPUT))
}

fn part2(input: &[u8]) -> usize {
    start_of_signal::<14>(input)
}

#[test]
fn part2_example() {
    assert_eq!(19, part2(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, part2(b"bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, part2(b"nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, part2(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, part2(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}

#[test]
#[ignore]
fn part2_verify() {
    assert_eq!(3425, part2(INPUT))
}

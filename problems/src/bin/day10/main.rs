use adventofcode2022_common::charcanvas::CharCanvas;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 \n{}", part2(INPUT));
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

struct Cpu {
    x: i32,
}

impl Cpu {
    fn execute(&mut self, instruction: Instruction) -> i32 {
        let x = self.x;
        if let Instruction::Add(dx) = instruction {
            self.x += dx;
        }
        x
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self { x: 1 }
    }
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    match line.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
        ["noop"] => vec![Instruction::Noop],
        ["addx", i] => vec![Instruction::Noop, Instruction::Add(i.parse().unwrap())],
        _ => unreachable!(),
    }
}

fn generate_signal(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .lines()
        .flat_map(parse_instructions)
        .scan(Cpu::default(), |cpu, instruction| {
            Some(cpu.execute(instruction))
        })
}

fn part1(input: &str) -> i32 {
    generate_signal(input)
        .enumerate()
        .skip(19)
        .step_by(40)
        .fold(0, |signal_strength, (cycle, reg)| {
            signal_strength + ((cycle + 1) as i32 * reg)
        })
}

#[test]
fn part1_example() {
    assert_eq!(13140, part1(EXAMPLE))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(14620, part1(INPUT))
}

fn part2(input: &str) -> String {
    let mut map = CharCanvas::with_size('.', [40, 6]);
    generate_signal(input)
        .zip(map.bounds.iter_points())
        .for_each(|(sprite_position, ray_position)| {
            map[ray_position] =
                if (sprite_position - 1..=sprite_position + 1).contains(&ray_position.x()) {
                    '#'
                } else {
                    '.'
                };
        });
    map.to_string()
}

#[test]
fn part2_example() {
    assert_eq!(include_str!("example_result.txt"), part2(EXAMPLE))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(include_str!("input_result.txt"), part2(INPUT))
}

#![feature(array_windows)]

use adventofcode2022_common::charcanvas::CharCanvas;
use adventofcode2022_common::vec2i::{Point, DOWN, DOWN_LEFT, DOWN_RIGHT};

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(INPUT));
    eprintln!("part2 {:?}", part2(INPUT));
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<Point>> + '_ {
    fn parse_coordinate(coordinate: &str) -> Point {
        let (x, y) = coordinate.split_once(',').unwrap();
        Point::new(x.parse().unwrap(), y.parse().unwrap())
    }

    fn parse_path(path: &str) -> Vec<Point> {
        path.split_terminator(" -> ")
            .map(parse_coordinate)
            .collect()
    }
    input.lines().map(parse_path)
}

fn render_grid(paths: impl Iterator<Item = Vec<Point>>) -> CharCanvas {
    let mut canvas = CharCanvas::new('.');
    for path in paths {
        for [start, end] in path.array_windows::<2>() {
            canvas.line(start, end, '#');
        }
    }

    canvas
}

enum SandPosition {
    Resting,
    Falling,
    Bottom,
}

#[inline]
fn fall(canvas: &CharCanvas, p: Point) -> (SandPosition, Point) {
    debug_assert!(canvas[p] == '.');
    if p.y() == canvas.bounds.bottom() {
        (SandPosition::Bottom, p)
    } else if canvas[p + DOWN] == '.' {
        (SandPosition::Falling, p + DOWN)
    } else if canvas[p + DOWN_LEFT] == '.' {
        (SandPosition::Falling, p + DOWN_LEFT)
    } else if canvas[p + DOWN_RIGHT] == '.' {
        (SandPosition::Falling, p + DOWN_RIGHT)
    } else {
        (SandPosition::Resting, p)
    }
}

fn drop_sand(canvas: &mut CharCanvas, mut p: Point) -> SandPosition {
    loop {
        let (sand_position, p2) = fall(canvas, p);
        p = p2;
        if !matches!(sand_position, SandPosition::Falling) {
            canvas[p2] = 'o';
            break sand_position;
        }
    }
}

const SAND_INGRESS: Point = Point::new(500, 0);

#[inline]
fn simulate(mut canvas: CharCanvas) -> impl Iterator<Item = SandPosition> {
    std::iter::from_fn(move || {
        (canvas[SAND_INGRESS] != 'o').then(|| drop_sand(&mut canvas, SAND_INGRESS))
    })
}

fn part1(input: &str) -> usize {
    let paths = parse_input(input);
    let canvas = render_grid(paths);

    simulate(canvas)
        .take_while(|sp| matches!(sp, SandPosition::Resting))
        .count()
}

fn part2(input: &str) -> usize {
    let paths = parse_input(input);
    let mut canvas = render_grid(paths);

    canvas.extend_to_point(&(canvas.bounds.bottom_right() + [0, 1]));

    simulate(canvas).count()
}

#[test]
fn part1_example() {
    assert_eq!(24, part1(EXAMPLE))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(1061, part1(INPUT))
}

#[test]
fn part2_example() {
    assert_eq!(93, part2(EXAMPLE))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(25055, part2(INPUT))
}

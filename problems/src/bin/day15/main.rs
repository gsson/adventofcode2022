#![feature(iter_array_chunks)]
#![feature(const_cmp, const_btree_len)]

use adventofcode2022_common::vec2i::Point;
use std::cmp::{max, min};
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

const SIDE: i32 = 4_000_000;

const INPUT: &str = include_str!("input.txt");
#[cfg(test)]
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    eprintln!("part1 {:?}", part1(2000000, INPUT));
    eprintln!("part2 {:?}", part2(SIDE, SIDE, INPUT));
}

fn parse_input(input: &str) -> impl Iterator<Item = (Point, Point)> + '_ {
    fn parse_coordinate(coordinate: &str) -> Point {
        let (x, y) = coordinate.split_once(", ").unwrap();
        let x = x.strip_prefix("x=").unwrap();
        let y = y.strip_prefix("y=").unwrap();
        Point::new(x.parse().unwrap(), y.parse().unwrap())
    }
    fn parse_line(line: &str) -> (Point, Point) {
        let (sensor, beacon) = line.split_once(": ").unwrap();
        let sensor = sensor.strip_prefix("Sensor at ").unwrap();
        let beacon = beacon.strip_prefix("closest beacon is at ").unwrap();
        (parse_coordinate(sensor), parse_coordinate(beacon))
    }
    input.lines().map(parse_line)
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum RangeBound {
    Lower,
    Upper,
}

#[derive(Default)]
struct RangeSet {
    ranges: BTreeSet<(i32, RangeBound)>,
}

impl RangeSet {
    fn remove_inner(&mut self, start: i32, end: i32) {
        let to_remove = self
            .ranges
            .range((start - 1, RangeBound::Upper)..=(end + 1, RangeBound::Lower))
            .copied()
            .collect::<Vec<_>>();

        for r in &to_remove {
            self.ranges.remove(r);
        }
    }

    #[inline]
    fn insert(&mut self, range: RangeInclusive<i32>) {
        let start = *range.start();
        let end = *range.end();

        let range_open = matches!(
            self.ranges.range(..=(start - 1, RangeBound::Lower)).last(),
            Some((_, RangeBound::Lower))
        );

        let range_closed = matches!(
            self.ranges.range((end + 1, RangeBound::Upper)..).next(),
            Some((_, RangeBound::Upper))
        );

        self.remove_inner(start, end);

        if !range_open {
            self.ranges.insert((start, RangeBound::Lower));
        }

        if !range_closed {
            self.ranges.insert((end, RangeBound::Upper));
        }
    }

    #[inline]
    const fn len(&self) -> usize {
        self.ranges.len() / 2
    }

    #[inline]
    fn contains(&self, p: i32) -> bool {
        matches!(
            self.ranges.range((p, RangeBound::Upper)..).next(),
            Some((_, RangeBound::Upper))
        )
    }

    fn into_iter(self) -> impl Iterator<Item = RangeInclusive<i32>> {
        self.ranges
            .into_iter()
            .array_chunks::<2>()
            .map(|[(start, _), (end, _)]| start..=end)
    }
}

struct SensorCoverage {
    x: i32,
    y: i32,
    range: i32,
}

impl SensorCoverage {
    fn new(sensor: Point, beacon: Point) -> Self {
        Self {
            range: sensor.manhattan_distance(&beacon),
            x: sensor.x(),
            y: sensor.y(),
        }
    }
    fn range_at_line(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let y_distance = self.y.abs_diff(y) as i32;
        if y_distance > self.range {
            return None;
        }
        let x_range = self.range - y_distance;
        let x1 = self.x - x_range;
        let x2 = self.x + x_range;
        Some(x1..=x2)
    }
}

#[inline]
const fn clamp_range<const LOW: i32, const HIGH: i32>(
    range: &RangeInclusive<i32>,
) -> Option<RangeInclusive<i32>> {
    let start = max(*range.start(), LOW);
    let end = min(*range.end(), HIGH);
    if start <= end {
        Some(RangeInclusive::new(start, end))
    } else {
        None
    }
}

const fn tuning_frequency(x: i32, y: i32) -> usize {
    x as usize * SIDE as usize + y as usize
}

fn sensor_coverage_at_line<const LOW: i32, const HIGH: i32>(
    line: i32,
    sensors: &[SensorCoverage],
) -> RangeSet {
    let mut coverage = RangeSet::default();
    for sensor_coverage in sensors {
        if let Some(range) = sensor_coverage.range_at_line(line) {
            if let Some(range) = clamp_range::<LOW, HIGH>(&range) {
                coverage.insert(range)
            }
        }
    }
    coverage
}

fn part1(y: i32, input: &str) -> usize {
    let mut beacons_at_line = BTreeSet::<i32>::new();
    let mut sensors = Vec::new();

    for (sensor, beacon) in parse_input(input) {
        sensors.push(SensorCoverage::new(sensor, beacon));
        if y == beacon.y() {
            beacons_at_line.insert(beacon.x());
        }
    }

    let coverage = sensor_coverage_at_line::<{ i32::MIN }, { i32::MAX }>(y, &sensors);

    let beacons = beacons_at_line
        .into_iter()
        .filter(|b| coverage.contains(*b))
        .count();

    let coverage = coverage.into_iter().map(|r| r.count()).sum::<usize>();
    coverage - beacons
}

fn part2(width: i32, height: i32, input: &str) -> usize {
    let sensors = parse_input(input)
        .map(|(sensor, beacon)| SensorCoverage::new(sensor, beacon))
        .collect::<Vec<_>>();

    for y in 0..height {
        let coverage = sensor_coverage_at_line::<0, SIDE>(y, &sensors);

        if coverage.len() > 1 {
            let x = coverage.into_iter().next().unwrap().end() + 1;
            return tuning_frequency(x, y);
        } else if !coverage.contains(0) {
            return tuning_frequency(0, y);
        } else if !coverage.contains(width) {
            return tuning_frequency(width, y);
        }
    }
    unreachable!()
}

#[test]
fn part1_example() {
    assert_eq!(26, part1(10, EXAMPLE))
}

#[ignore]
#[test]
fn part1_verify() {
    assert_eq!(5147333, part1(2000000, INPUT))
}

#[test]
fn part2_example() {
    assert_eq!(56000011, part2(20, 20, EXAMPLE))
}

#[ignore]
#[test]
fn part2_verify() {
    assert_eq!(13734006908372, part2(SIDE, SIDE, INPUT))
}

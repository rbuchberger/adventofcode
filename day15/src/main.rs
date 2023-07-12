use regex::Regex;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    sensor: (isize, isize),
    beacon: (isize, isize),
}

impl From<&str> for Pair {
    fn from(string: &str) -> Self {
        let results = Regex::new(
            r"^Sensor at x=([\d\-]+), y=([\d\-]+): closest beacon is at x=([\d\-]+), y=([\d\-]+)$",
        )
        .unwrap()
        .captures(string)
        .unwrap();

        let mut results = results
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse().unwrap());

        let sensor = (results.next().unwrap(), results.next().unwrap());
        let beacon = (results.next().unwrap(), results.next().unwrap());

        Self { sensor, beacon }
    }
}

impl Pair {
    fn distance(&self) -> isize {
        (self.sensor.0 - self.beacon.0).abs() + (self.sensor.1 - self.beacon.1).abs()
    }

    fn row_coverage(&self, row: isize) -> Option<RangeInclusive<isize>> {
        let vertical_dist = (self.sensor.1 - row).abs();

        if vertical_dist > self.distance() {
            None
        } else {
            let width = self.distance() - vertical_dist;
            Some(self.sensor.0 - width..=self.sensor.0 + width)
        }
    }
}

fn pairs(filename: &str) -> Vec<Pair> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(Pair::from)
        .collect()
}

fn map_coverage(pairs: Vec<Pair>, row: isize) -> Vec<RangeInclusive<isize>> {
    let mut coverage = pairs
        .iter()
        .filter_map(|pair| pair.row_coverage(row))
        .collect::<Vec<_>>();

    coverage.sort_by_key(|range| *range.start());

    let coverage = coverage
        .iter()
        .fold(Vec::new(), |mut acc: Vec<RangeInclusive<isize>>, curr| {
            if let Some(prev) = acc.last() {
                match (
                    (&(prev.end() + 1) >= curr.start()),
                    prev.contains(curr.end()),
                ) {
                    (false, false) => acc.push(curr.clone()), // new, separate section
                    (true, true) => (),                       // completely contained
                    (false, true) => unreachable!(),          // we already sorted by start

                    // need to extend the last range
                    (true, false) => {
                        let previous = prev.clone();

                        acc.pop();
                        acc.push(previous.start().clone()..=curr.end().clone());
                    }
                }
            } else {
                acc.push(curr.clone())
            }

            acc
        });

    coverage
}

fn part1(filename: &str, row: isize) -> isize {
    let coverage = map_coverage(pairs(filename), row);

    coverage.iter().map(|r| r.end() - r.start()).sum()
}

const TUNING_FREQ: isize = 4_000_000;

fn part2(filename: &str, bound: isize) -> isize {
    let pairs = pairs(filename);

    let (y, x) = (0..=bound)
        .into_iter()
        .map(|row| map_coverage(pairs.clone(), row))
        .enumerate()
        .find(|(_, coverage)| {
            coverage.len() > 1 || coverage[0].start() > &0 || coverage[0].end() < &bound
        })
        .unwrap();

    ((x[0].end() + 1) * TUNING_FREQ) + y as isize
}

fn main() {
    dbg!(part1("example", 10));
    dbg!(part1("input", 2_000_000));

    dbg!(part2("example", 20));
    dbg!(part2("input", 4_000_000));
}

use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(a: (isize, isize), b: (isize, isize)) -> Self {
        match (b.0.cmp(&a.0), b.1.cmp(&a.1)) {
            (Equal, Greater) => Self::Down,
            (Equal, Less) => Self::Up,
            (Less, Equal) => Self::Left,
            (Greater, Equal) => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn step(&self, point: (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (point.0, point.1 - 1),
            Self::Down => (point.0, point.1 + 1),
            Self::Left => (point.0 - 1, point.1),
            Self::Right => (point.0 + 1, point.1),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Fall {
    None,
    Limited((isize, isize)),
    Unlimited(isize),
}

impl Fall {
    fn new(cave: &BTreeMap<isize, BTreeSet<isize>>, point: (isize, isize)) -> Self {
        let col = cave.get(&point.0);

        if col.is_none() {
            return Self::Unlimited(point.0);
        }

        match col.unwrap().range((&point.1 + 1)..).next() {
            Some(&limit) => {
                if limit > &point.1 + 1 {
                    Self::Limited((point.0, limit - 1))
                } else {
                    Self::None
                }
            }

            None => Self::Unlimited(point.0),
        }
    }
}

fn main() {
    let text = std::fs::read_to_string("input").unwrap();

    // Map cave
    let mut cave: BTreeMap<isize, BTreeSet<isize>> = BTreeMap::new();
    for path in text.lines().map(|line| {
        line.split(" -> ")
            .map(|point| {
                let mut point = point.split(",").map(|i| i.parse().unwrap());

                (point.next().unwrap(), point.next().unwrap())
            })
            .collect::<Vec<(isize, isize)>>()
    }) {
        let mut path = path.iter();
        let mut start = path.next().unwrap();

        for node in path {
            let mut current = *start;
            let direction = Direction::new(*start, *node);

            loop {
                cave.entry(current.0)
                    .or_insert(BTreeSet::new())
                    .insert(current.1);

                if current == *node {
                    break;
                }

                current = direction.step(current);
            }

            start = node;
        }
    }

    let bottom = cave
        .iter()
        .map(|(_, col)| col.last().unwrap_or(&0))
        .max()
        .unwrap()
        + 1;

    // Drop sand
    'all_grains: for i in 1.. {
        let mut grain = (500, 0);

        'this_grain: loop {
            match [grain, (grain.0 - 1, grain.1), (grain.0 + 1, grain.1)]
                .iter()
                .map(|o| Fall::new(&cave, *o))
                .find(|o| *o != Fall::None)
            {
                Some(Fall::Limited(new_point)) => grain = new_point,

                Some(Fall::Unlimited(col)) => {
                    cave.entry(col).or_insert(BTreeSet::new()).insert(bottom);

                    break 'this_grain;
                }

                None => {
                    if grain.1 == 0 {
                        println!("Result: {}", i);
                        break 'all_grains;
                    }

                    cave.entry(grain.0).and_modify(|c| {
                        c.insert(grain.1);
                    });

                    break 'this_grain;
                }

                _ => unreachable!(),
            }
        }
    }
}

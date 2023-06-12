use std::ops::RangeInclusive;

// Example problem has 2 overlaps
const _EXAMPLE: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

#[derive(Debug, Eq, PartialEq, Clone)]
struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    fn parse(input: &str) -> Self {
        let mut parts = input.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        if parts.next().is_some() {
            panic!("Invalid range: {}", input);
        }

        Self { start, end }
    }

    // Returns true if either range fully contains the other
    fn contains(&self, other: &Self) -> bool {
        self == other || self.start.cmp(&other.start) != self.end.cmp(&other.end)
    }

    fn overlaps(&self, other: &Self) -> bool {
        let rng: RangeInclusive<usize> = self.clone().into();

        self.contains(other) || rng.contains(&other.start) || rng.contains(&other.end)
    }
}

impl Into<RangeInclusive<usize>> for Assignment {
    fn into(self) -> RangeInclusive<usize> {
        self.start..=self.end
    }
}

fn main() {
    let containers = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut line = line.split(',');
            let ranges = (
                Assignment::parse(line.next().unwrap()),
                Assignment::parse(line.next().unwrap()),
            );

            if line.next().is_some() {
                panic!("Invalid line");
            }

            ranges
        })
        .filter(|&(ref a, ref b)| a.overlaps(b))
        .count();

    println!("{}", containers);
}

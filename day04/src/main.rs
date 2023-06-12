// Example problem has 2 overlaps
const _EXAMPLE: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

#[derive(Debug, Eq, PartialEq)]
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
        // Return only ranges which overlap.
        // This means that the ranges are either equal, or the start & end of one
        // are not both greater than or less than the start & end of the other.
        .filter(|&(ref a, ref b)| a == b || a.start.cmp(&b.start) != a.end.cmp(&b.end))
        .count();

    println!("{}", containers);
}

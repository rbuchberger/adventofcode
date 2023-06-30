use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

const _EXAMPLE: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

fn main() {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);

    // let text = _EXAMPLE;
    let text = fs::read_to_string("input").unwrap();

    for line in text.trim().lines() {
        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.trim().parse().unwrap();

        for _ in 0..distance {
            match direction {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => panic!("Unknown direction: {}", direction),
            }

            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail.0 += match head.0.cmp(&tail.0) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };

                tail.1 += match head.1.cmp(&tail.1) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
            }

            visited.insert(tail);
        }
    }

    dbg!(visited.len());
}

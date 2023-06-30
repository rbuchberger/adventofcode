use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

const _EXAMPLE: &str = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

fn main() {
    let mut rope = vec![(0, 0); 10];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(*rope.last().unwrap());

    // let text = _EXAMPLE;
    let text = fs::read_to_string("input").unwrap();

    for line in text.trim().lines() {
        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.trim().parse().unwrap();

        for _ in 0..distance {
            match direction {
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                _ => panic!("Unknown direction: {}", direction),
            }

            for i in 1..rope.len() {
                let leader = rope[i - 1];
                let mut follower = &mut rope[i];

                if (leader.0 - follower.0).abs() > 1 || (leader.1 - follower.1).abs() > 1 {
                    follower.0 += match leader.0.cmp(&follower.0) {
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                    };

                    follower.1 += match leader.1.cmp(&follower.1) {
                        Ordering::Less => -1,
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                    };
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    dbg!(visited.len());
}

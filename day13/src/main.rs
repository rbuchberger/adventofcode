use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Val {
    I(i8),
    L(Vec<Val>),
}

impl Val {
    fn parse(s: &str) -> (Self, usize) {
        let mut list = Vec::new();
        let mut current_int = String::new();
        let mut skip_to = 0;
        let mut length = 0;

        for (i, c) in s.chars().skip(1).enumerate() {
            if i < skip_to {
                continue;
            }

            match c {
                '0'..='9' => current_int.push(c),
                ',' => {
                    if !current_int.is_empty() {
                        list.push(Val::I(current_int.parse().unwrap()));
                        current_int.clear();
                    };
                }
                '[' => {
                    let (sublist, length) = Self::parse(&s[i + 1..]);
                    list.push(sublist);
                    skip_to = length + i + 2;
                }
                ']' => {
                    if !current_int.is_empty() {
                        list.push(Val::I(current_int.parse().unwrap()));
                    }
                    length = i;
                    break;
                }
                _ => panic!("Invalid character"),
            };
        }

        (Val::L(list), length)
    }
}

impl From<&str> for Val {
    fn from(s: &str) -> Self {
        Self::parse(s).0
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Val::I(a), &Val::I(b)) => Some(i8::cmp(&a, &b)),
            (Val::L(a), Val::L(b)) => a.partial_cmp(b),
            (Val::I(_), Val::L(_)) => Val::L(vec![self.clone()]).partial_cmp(other),
            (Val::L(_), Val::I(_)) => self.partial_cmp(&Val::L(vec![other.clone()])),
        }
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        println!("cmp {:?} and {:?}", self, other);
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let text = fs::read_to_string("input").unwrap();

    let result1: usize = text
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.into()).collect::<Vec<Val>>())
        .enumerate()
        .filter(|(_, p)| p[0] <= p[1])
        .map(|(i, _)| i + 1) // not zero indexed!
        .sum();

    println!("Part 1 result: {}", result1);

    let dividers = vec![
        Val::L(vec![Val::L(vec![Val::I(2)])]),
        Val::L(vec![Val::L(vec![Val::I(6)])]),
    ];

    let mut packets2 = text
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.into())
        .collect::<Vec<Val>>();

    packets2.append(&mut dividers.clone());
    packets2.sort();

    let result2: usize = packets2
        .iter()
        .enumerate()
        .filter(|(_, p)| dividers.contains(p))
        .map(|(i, _)| i + 1)
        .product();

    println!("Part 2 result: {:?}", result2);
}

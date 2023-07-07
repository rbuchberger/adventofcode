use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Value {
    Integer(i8),
    List(Vec<Value>),
}

impl Value {
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
                        list.push(Value::Integer(current_int.parse().unwrap()));
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
                        list.push(Value::Integer(current_int.parse().unwrap()));
                    }
                    length = i;
                    break;
                }
                _ => panic!("Invalid character"),
            };
        }

        (Value::List(list), length)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::parse(s).0
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Value::Integer(a), &Value::Integer(b)) => Some(i8::cmp(&a, &b)),
            (Value::List(a), Value::List(b)) => a.partial_cmp(b),
            (Value::Integer(_), Value::List(_)) => {
                Value::List(vec![self.clone()]).partial_cmp(other)
            }
            (Value::List(_), Value::Integer(_)) => {
                self.partial_cmp(&Value::List(vec![other.clone()]))
            }
        }
    }
}

fn main() {
    let text = fs::read_to_string("input").unwrap();

    let result: usize = text
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.into()).collect::<Vec<Value>>())
        .enumerate()
        .filter(|(_, p)| p[0] <= p[1])
        .map(|(i, _)| i + 1) // not zero indexed!
        .sum();

    println!("Result: {}", result);
}

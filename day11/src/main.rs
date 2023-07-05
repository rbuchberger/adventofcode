use regex::Regex;
use std::fs;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn parse(arg: &str, val: &str) -> Self {
        match (arg, val) {
            ("*", "old") => Operation::Square,
            ("*", value) => Operation::Multiply(value.parse::<u64>().unwrap()),
            ("+", value) => Operation::Add(value.parse::<u64>().unwrap()),
            _ => panic!("Unknown operation"),
        }
    }

    fn exec(&self, input: u64) -> u64 {
        match self {
            Operation::Add(val) => input + val,
            Operation::Multiply(val) => input * val,
            Operation::Square => input * input,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    index: usize,
    items: Vec<u64>,
    operation: Operation,
    test_val: u64,
    pass_recipient: usize,
    fail_recipient: usize,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let parser = Regex::new(
            &[
                r"Monkey (\d+):",
                r"  Starting items: (.+)",
                r"  Operation: new = old (\W) (.+)",
                r"  Test: divisible by (\d+)",
                r"    If true: throw to monkey (\d+)",
                r"    If false: throw to monkey (\d+)",
            ]
            .join("\n"),
        )
        .unwrap();

        let matches = parser.captures(input).unwrap();
        let mut matches = matches.iter().map(|m| m.unwrap().as_str()).skip(1);
        let index = matches.next().unwrap().parse::<usize>().unwrap();

        let items = matches
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let operation = Operation::parse(matches.next().unwrap(), matches.next().unwrap());

        let test_val = matches.next().unwrap().parse().unwrap();

        let mut matches = matches.map(|s| s.parse::<usize>().unwrap());
        let pass_recipient = matches.next().unwrap();
        let fail_recipient = matches.next().unwrap();

        Monkey {
            index,
            items,
            operation,
            test_val,
            pass_recipient,
            fail_recipient,
        }
    }
}

fn main() {
    let text = fs::read_to_string("input").unwrap();
    let monkeys = text
        .trim()
        .split("\n\n")
        .map(Monkey::parse)
        .collect::<Vec<_>>();

    let mut hands = monkeys
        .iter()
        .map(|m| m.items.clone())
        .collect::<Vec<Vec<_>>>();

    let all_test_product = monkeys.iter().map(|m| m.test_val).product::<u64>();

    let mut counters = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for monkey in &monkeys {
            for item in hands[monkey.index].clone() {
                counters[monkey.index] += 1;

                // This is the key to not overflowing. We don't have to store the whole number,
                // simply its mod of the product of all test values. It's some math thing, I didn't
                // come up with it.
                let new_item = monkey.operation.exec(item) % all_test_product;

                let recipient = if new_item % monkey.test_val == 0 {
                    monkey.pass_recipient
                } else {
                    monkey.fail_recipient
                };

                hands[recipient].push(new_item);
            }

            hands[monkey.index].clear();
        }
    }

    counters.sort();

    let monkey_business_level =
        counters[monkeys.len() - 1] as f64 * counters[monkeys.len() - 2] as f64;

    dbg!(monkey_business_level);
}

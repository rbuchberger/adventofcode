use std::fs;

fn next_cycle(register: isize, cycle_nr: &mut isize, results: &mut Vec<isize>) {
    if (*cycle_nr - 20) % 40 == 0 {
        results.push(register * *cycle_nr);
    }

    *cycle_nr += 1;
}

fn main() {
    let mut register = 1;
    let mut cycle = 1;
    let mut results = Vec::new();

    let text = fs::read_to_string("input").unwrap();
    for instruction in text.trim().lines() {
        next_cycle(register, &mut cycle, &mut results);

        match instruction.split_whitespace().collect::<Vec<_>>()[..] {
            ["noop"] => (),
            ["addx", val] => {
                next_cycle(register, &mut cycle, &mut results);
                register += val.parse::<isize>().unwrap();
            }
            _ => panic!("Unknown instruction"),
        }
    }

    dbg!(results.iter().sum::<isize>());
}

use std::fs;

const SCREEN_WIDTH: isize = 40;
const SCREEN_HEIGHT: isize = 6;

fn next_cycle(register: isize, cycle_nr: &mut isize, pickles: &mut Vec<Vec<char>>) {
    let row = *cycle_nr / SCREEN_WIDTH;
    let col = *cycle_nr % SCREEN_WIDTH;

    let pickle = if (register - col).abs() < 2 {
        '█'
    } else {
        ' '
    };

    pickles[row as usize].push(pickle);

    *cycle_nr += 1;
}

fn main() {
    let mut register = 1;
    let mut cycle_nr = 0;
    let mut pickles = vec![Vec::new(); SCREEN_HEIGHT as usize];

    let text = fs::read_to_string("input").unwrap();
    for instruction in text.trim().lines() {
        next_cycle(register, &mut cycle_nr, &mut pickles);

        match instruction.split_whitespace().collect::<Vec<_>>()[..] {
            ["noop"] => (),
            ["addx", val] => {
                next_cycle(register, &mut cycle_nr, &mut pickles);
                register += val.parse::<isize>().unwrap();
            }
            _ => panic!("Unknown instruction"),
        }
    }

    println![
        "{}",
        pickles
            .iter()
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    ];
}

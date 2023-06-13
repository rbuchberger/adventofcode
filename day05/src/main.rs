// Actual input (sample)
//             [C]         [N] [R]
// [J] [T]     [H]         [P] [L]
// [F] [S] [T] [B]         [M] [D]
// [C] [L] [J] [Z] [S]     [L] [B]
// [N] [Q] [G] [J] [J]     [F] [F] [R]
// [D] [V] [B] [L] [B] [Q] [D] [M] [T]
// [B] [Z] [Z] [T] [V] [S] [V] [S] [D]
// [W] [P] [P] [D] [G] [P] [B] [P] [V]
//  1   2   3   4   5   6   7   8   9
//
// move 4 from 9 to 6
// move 7 from 2 to 5
// move 3 from 5 to 2
// move 2 from 2 to 1
// move 2 from 8 to 4
// move 1 from 6 to 9
// move 1 from 9 to 4
// move 7 from 1 to 2

use std::fs;

// From example problem
const _SAMPLE: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

#[derive(Debug)]
struct Instruction {
    source: usize,
    dest: usize,
    count: usize,
}

impl Instruction {
    // "move (count) from (source) to (dest)"
    fn parse(string: &str) -> Self {
        let mut parts = string.split_whitespace().skip(1).step_by(2);

        let count = parts.next().unwrap().parse().unwrap();
        let source = parts.next().unwrap().parse().unwrap();
        let dest = parts.next().unwrap().parse().unwrap();

        Self {
            count,
            source,
            dest,
        }
    }

    fn execute(self, stacks: &mut Vec<Vec<char>>) {
        for _ in 0..self.count {
            let c = stacks[self.source - 1].pop().unwrap();

            stacks[self.dest - 1].push(c);
        }
    }
}

fn main() {
    // Split on blank line, starting state is first set of lines, instructions are second
    // let mut input = _SAMPLE.splitn(2, "\n\n");
    let input = fs::read_to_string("input").unwrap();
    let mut input = input.splitn(2, "\n\n");

    // Parse strings into vectors of chars.
    // The skip() and step_by() calls pluck values out of the ascii art.
    // [C]         [N]    [R] => ['C', ' ', ' ', 'N', ' ', 'R']
    let rows = input.next().unwrap();
    let rows = rows
        .lines()
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Initialize empty stacks
    let mut stacks: Vec<_> = vec![Vec::new(); rows.first().unwrap().len()];

    // Transpose rows to stacks, bottom to top. First row is indexes, so skip.
    for row in rows.iter().rev().skip(1) {
        for (i, char) in row.iter().enumerate() {
            if char.is_whitespace() {
                continue;
            }

            stacks[i].push(*char);
        }
    }

    let instructions = input.next().unwrap().trim();
    instructions
        .lines()
        .map(Instruction::parse)
        .for_each(|i| i.execute(&mut stacks));

    // See what's on top
    let tops = stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>();

    println!("{}", tops);
}

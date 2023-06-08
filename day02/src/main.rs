// First char is opponent, second is player
//
// Rock: A, X, scores 1
// Paper: B, Y, scores 2
// Scissors: C, Z, scores 3
//
// Lose: 0 points
// Draw: 3 points
// Win: 6 points

fn main() {
    let text = std::fs::read_to_string("input").unwrap();

    let score: i32 = text
        .trim()
        .lines()
        .map(|line| Round::build(line).score())
        .sum();

    println!("Score: {}", score);
}

struct Round {
    player: Shape,
    opponent: Shape,
}

impl Round {
    fn score(&self) -> i32 {
        Outcome::build(&self.opponent, &self.player) as i32 + self.player as i32
    }

    fn build(line: &str) -> Self {
        let chars = line.split(" ").collect::<Vec<&str>>();

        Round {
            opponent: Shape::parse(chars[0]),
            player: Shape::parse(chars[1]),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn parse(code: &str) -> Self {
        match code {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid shape"),
        }
    }

    fn beats(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl Outcome {
    fn build(opponent: &Shape, player: &Shape) -> Self {
        if player == opponent {
            Outcome::Draw
        } else if player.beats() == *opponent {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }
}

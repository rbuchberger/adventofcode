// First char is opponent's play, second is desired outcome
//
// Rock: A scores 1
// Paper: B, scores 2
// Scissors: C, scores 3
//
// Lose: 0 points, X
// Draw: 3 points, Y
// Win: 6 points, Z

fn main() {
    let score: i32 = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut codes = line.split(" ");
            let (opponent_move, outcome) = (
                Shape::parse(codes.next().unwrap()),
                Outcome::parse(codes.next().unwrap()),
            );

            let player_move = match outcome {
                Outcome::Lose => opponent_move.beats(),
                Outcome::Draw => opponent_move,
                // Kinda lazy but it works
                Outcome::Win => opponent_move.beats().beats(),
            };

            outcome as i32 + player_move as i32
        })
        .sum();

    println!("Score: {}", score);
}

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn parse(code: &str) -> Self {
        match code {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid shape code"),
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
    fn parse(code: &str) -> Self {
        match code {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome code"),
        }
    }
}

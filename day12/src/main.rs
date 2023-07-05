use std::collections::{HashSet, VecDeque};

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
enum Point {
    Start,
    End,
    P(usize),
}

impl Point {
    fn parse(c: char) -> Self {
        match c {
            'S' => Self::Start,
            'E' => Self::End,
            _ => Self::P(('a'..='z').position(|x| x == c).unwrap()),
        }
    }

    fn height(&self) -> usize {
        match self {
            Self::Start => 0,
            Self::End => 25,
            Self::P(h) => *h,
        }
    }

    fn can_reach(&self, candidate: &Self) -> bool {
        candidate.height() as isize - self.height() as isize <= 1
    }
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    fn apply(&self, old: &Coord, max: &Coord) -> Option<Coord> {
        let mut x = old.x;
        let mut y = old.y;

        match self {
            Self::Up => {
                if y == max.y {
                    return None;
                }

                y += 1;
            }

            Self::Down => {
                if y == 0 {
                    return None;
                }

                y -= 1;
            }

            Self::Left => {
                if old.x == 0 {
                    return None;
                }

                x -= 1;
            }
            Self::Right => {
                if x == max.x {
                    return None;
                }

                x += 1;
            }
        }

        Some(Coord { x, y })
    }
}

struct Map(Vec<Vec<Point>>);

impl Map {
    fn get(&self, coord: &Coord) -> &Point {
        &self.0[coord.y][coord.x]
    }

    fn parse(text: &str) -> Self {
        Map(text
            .trim()
            .lines()
            .map(|line| line.chars().map(Point::parse).collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    fn max(&self) -> Coord {
        Coord {
            x: self.0[0].len() - 1,
            y: self.0.len() - 1,
        }
    }

    fn start(&self) -> Coord {
        let y = self
            .0
            .iter()
            .position(|line| line.contains(&Point::Start))
            .unwrap();
        let x = self.0[y].iter().position(|p| *p == Point::Start).unwrap();

        Coord { x, y }
    }
}

fn main() {
    let text = std::fs::read_to_string("input").unwrap();
    let map = Map::parse(&text);
    let map_bound = map.max();

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut queue = VecDeque::from([vec![map.start()]]);

    while queue.len() > 0 {
        let current_path = queue.pop_front().unwrap();
        let current_coords = current_path.last().unwrap();

        let previously_visited = !visited.insert(current_coords.clone());
        if previously_visited {
            continue;
        }

        let current_point = map.get(current_coords);

        if current_point == &Point::End {
            println!("Shortest path found: {:?}", current_path.len() - 1);
            break;
        }

        for coords in Move::ALL
            .iter()
            .filter_map(|m| m.apply(current_coords, &map_bound))
            .filter(|coords| current_point.can_reach(map.get(&coords)))
        {
            // This clone is pretty inefficient, but who cares
            let mut new_path = current_path.clone();
            new_path.push(coords);
            queue.push_back(new_path);
        }
    }
}

fn main() {
    let chars = ('a'..='z').chain('A'..='Z').collect::<Vec<char>>();

    let text = std::fs::read_to_string("input").unwrap();

    let result: usize = text
        .trim()
        .lines()
        .map(|pack| {
            let (comp1, comp2) = pack.split_at(pack.len() / 2);

            let dupe = comp1.chars().find(|c| comp2.contains(*c)).unwrap();
            let position = chars.iter().position(|c| *c == dupe).unwrap();

            position + 1
        })
        .sum();

    println!("Result: {}", result);
}

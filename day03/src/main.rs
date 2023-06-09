fn main() {
    let text = std::fs::read_to_string("input").unwrap();

    let duplicate_priorities: usize = text
        .trim()
        .lines()
        .map(|pack| {
            let (comp1, comp2) = pack.split_at(pack.len() / 2);
            let dupe = comp1.chars().find(|c| comp2.contains(*c)).unwrap();

            get_priority(dupe)
        })
        .sum();

    println!("Duplicate Priorities: {}", duplicate_priorities);

    let badge_priorities: usize = text
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .windows(3)
        .step_by(3)
        .map(|group| {
            let [member1, member2, member3] = group else { panic!("Group of not 3") };

            let badge = member1
                .chars()
                .find(|c| member2.contains(*c) && member3.contains(*c))
                .unwrap();

            get_priority(badge)
        })
        .sum();

    println!("Badge Priorities: {}", badge_priorities);
}

fn get_priority(item: char) -> usize {
    let chars = ('a'..='z').chain('A'..='Z').collect::<Vec<char>>();
    let position = chars.iter().position(|c| *c == item).unwrap();

    position + 1
}

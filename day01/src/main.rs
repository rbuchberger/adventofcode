fn main() {
    let text = std::fs::read_to_string("input").unwrap();

    let lines = text.lines().collect::<Vec<&str>>();

    let mut elves = lines
        .rsplit(|e| *e == "")
        .map(|e| {
            e.into_iter()
                .map(|i| i.parse::<isize>().unwrap())
                .sum::<isize>()
        })
        .collect::<Vec<isize>>();

    elves.sort();
    elves.reverse();

    let biggest_elf = elves.first().unwrap();
    println!("{}", biggest_elf);

    let top_three = elves.iter().take(3).sum::<isize>();
    println!("{}", top_three);
}

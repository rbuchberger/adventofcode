fn main() {
    let text = std::fs::read_to_string("input").unwrap();

    let biggest_elf = text
        .lines()
        .collect::<Vec<&str>>()
        .rsplit(|e| *e == "")
        .map(|e| {
            e.into_iter()
                .map(|i| i.parse::<isize>().unwrap())
                .sum::<isize>()
        })
        .max()
        .unwrap();

    println!("{}", biggest_elf);
}

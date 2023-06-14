use std::fs;

const _EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"; // marker is 7th char

fn main() {
    let input = fs::read_to_string("input")
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    let marker = input.windows(4).enumerate().find(|(_, window)| {
        // dedup only works on consecutive dupes, so needs to be sorted first
        let mut window = window.to_vec();
        window.sort();
        window.dedup();
        window.len() == 4
    });

    println!("{:?}", marker.unwrap().0 + 4);
}

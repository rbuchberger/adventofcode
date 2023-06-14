use std::fs;

const _EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"; // marker is 7th char
const WINDOW_SIZE: usize = 14;

fn main() {
    let input = fs::read_to_string("input")
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    let marker = input.windows(WINDOW_SIZE).enumerate().find(|(_, window)| {
        // dedup only works on consecutive dupes, so needs to be sorted first
        let mut window = window.to_vec();
        window.sort();
        window.dedup();
        window.len() == WINDOW_SIZE
    });

    println!("{:?}", marker.unwrap().0 + WINDOW_SIZE);
}

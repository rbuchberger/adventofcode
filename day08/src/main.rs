const _EXAMPLE: &str = "
30373
25512
65332
33549
35390
";

fn main() {
    // let text = _EXAMPLE;
    let text = std::fs::read_to_string("input").unwrap();

    let grid: Vec<Vec<u32>> = text
        .trim()
        .lines()
        .map(|r| r.chars().map(|c| c.into()).collect())
        .collect();

    let vis_count = grid
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(col_index, tree)| {
                    // split_at includes the item at the current index in the right side; this
                    // pattern excludes it. Same applies to the column split.
                    let (left, [_, right @ ..]) = row.split_at(*col_index) else {
                        panic!("split_at failed");
                    };

                    let col = grid.iter().map(|r| r[*col_index]).collect::<Vec<_>>();
                    let (up, [_, down @ ..]) = col
                        .split_at(row_index) else {
                            panic!("split_at failed");
                        };

                    // Is there any direction where all trees are smaller than the current tree?
                    [left, right, up, down]
                        .iter()
                        .any(|dir| dir.iter().all(|&t| t < **tree))
                })
                .count()
        })
        .sum::<usize>();

    dbg!(vis_count);
}

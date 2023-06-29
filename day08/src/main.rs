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

    let max_scenic_score = grid
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(col_index, tree)| {
                    let ( left, [_, right @ ..]) = row.split_at(col_index) else {
                        panic!("split_at failed");
                    };

                    let col = grid.iter().map(|r| r[col_index]).collect::<Vec<_>>();
                    let ( up, [_, down @ ..]) = col
                        .split_at(row_index) else {
                            panic!("split_at failed");
                        };

                    let mut left = left.to_owned();
                    left.reverse();
                    let mut up = up.to_owned();
                    up.reverse();

                    let right: Vec<_> = right.into();
                    let down: Vec<_> = down.into();

                    [left, right, up, down]
                        .iter()
                        .map(|dir| {
                            if let Some(result) = dir.iter().position(|&t| t >= *tree) {
                                result + 1
                            } else {
                                dir.len()
                            }
                        })
                        .product::<usize>()
                })
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap();

    dbg!(vis_count);
    dbg!(max_scenic_score);
}

use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/08");

#[time_run2("08")]
fn main() {
    visible_trees(INPUT)
}

#[derive(Clone, Default)]
struct Tree {
    size: i8,
    visible: bool,
    left_score: u64,
    right_score: u64,
    top_score: u64,
    bottom_score: u64,
}

impl Tree {
    fn new(size: i8) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }
}

fn visible_trees(i: &str) -> (String, String) {
    let mut grid: Vec<Vec<Tree>> = i
        .lines()
        .map(|line| {
            let l: Vec<Tree> = line
                .chars()
                .map(|c| Tree::new(c.to_string().parse::<i8>().unwrap()))
                .collect();
            l
        })
        .collect();

    let mut seen_tree_sizes: Vec<i8> = Vec::with_capacity(grid.len().max(grid[0].len()));

    // Looking from the left
    for (_, trees) in grid.iter_mut().enumerate() {
        let mut max_size = -1;
        seen_tree_sizes.clear();
        for (_, tree) in trees.iter_mut().enumerate() {
            if tree.size > max_size {
                max_size = tree.size;
                tree.visible = true;
            }

            let mut left_score = 0;
            for size in seen_tree_sizes.iter().rev() {
                left_score += 1;
                if tree.size <= *size {
                    break;
                }
            }
            tree.left_score = left_score;

            seen_tree_sizes.push(tree.size);
        }
    }

    // Looking from the right
    for (_, trees) in grid.iter_mut().enumerate() {
        let mut max_size = -1;
        seen_tree_sizes.clear();

        for (_, tree) in trees.iter_mut().enumerate().rev() {
            if tree.size > max_size {
                max_size = tree.size;
                tree.visible = true;
            }
            let mut right_score = 0;
            for size in seen_tree_sizes.iter().rev() {
                right_score += 1;
                if tree.size <= *size {
                    break;
                }
            }
            tree.right_score = right_score;
            seen_tree_sizes.push(tree.size);
        }
    }

    // Top to bottom
    for i in 0..grid[0].len() {
        let mut max_size = -1;
        seen_tree_sizes.clear();

        for line in &mut grid {
            let tree_size = line[i].size;
            if tree_size > max_size {
                max_size = tree_size;
                line[i].visible = true;
            }

            let mut top_score = 0;
            for size in seen_tree_sizes.iter().rev() {
                top_score += 1;
                if tree_size <= *size {
                    break;
                }
            }
            line[i].top_score = top_score;

            seen_tree_sizes.push(tree_size);
        }
    }

    // Finally bottom to top
    for i in 0..grid[0].len() {
        let mut max_size = -1;
        seen_tree_sizes.clear();
        for j in (0..grid.len()).rev() {
            let tree_size = grid[j][i].size;
            if tree_size > max_size {
                max_size = tree_size;
                grid[j][i].visible = true;
            }
            let mut bottom_score = 0;
            for size in seen_tree_sizes.iter().rev() {
                bottom_score += 1;
                if tree_size <= *size {
                    break;
                }
            }
            grid[j][i].bottom_score = bottom_score;

            seen_tree_sizes.push(tree_size);
        }
    }

    // Part 1
    let mut total1 = 0;
    for line in grid.iter() {
        for tree in line {
            if tree.visible {
                total1 += 1;
            }
        }
    }

    let max_scenic_score = grid
        .iter()
        .flatten()
        .map(|tree| tree.left_score * tree.right_score * tree.top_score * tree.bottom_score)
        .max()
        .unwrap();

    (total1.to_string(), max_scenic_score.to_string())
}

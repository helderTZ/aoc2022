const HEIGHT_MAP: &'static str = include_str!("input.txt");

// TODO: would be faster to start around the tree than from the sides
// TODO: save the max height tree in the path, that way we can save alot of
//       comparisons (compare with the largest until now, we can stop there
//       if it is not visible)

fn is_visible_row(row: usize, col: usize, height_mat: &Vec<Vec<usize>>) -> bool {
    let mut visible_left_right = true;
    let tree_under_test = height_mat[row][col];

    for c in 0..col {
        let curr_tree = height_mat[row][c];
        visible_left_right &= tree_under_test > curr_tree;
        if !visible_left_right {
            break;
        }
    }

    if visible_left_right {
        return visible_left_right;
    }

    let mut visible_right_left = true;
    for c in (col+1..height_mat[row].len()).rev() {
        let curr_tree = height_mat[row][c];
        visible_right_left &= tree_under_test > curr_tree;
        if !visible_right_left {
            break;
        }
    }

    visible_left_right | visible_right_left
}

fn is_visible_col(row: usize, col: usize, height_mat: &Vec<Vec<usize>>) -> bool {
    let mut visible_up_down = true;
    let tree_under_test = height_mat[row][col];

    for r in 0..row {
        let curr_tree = height_mat[r][col];
        visible_up_down &= tree_under_test > curr_tree;
        if !visible_up_down {
            break;
        }
    }

    if visible_up_down {
        return visible_up_down;
    }

    let mut visible_down_up = true;
    for r in (row+1..height_mat.len()).rev() {
        let curr_tree = height_mat[r][col];
        visible_down_up &= tree_under_test > curr_tree;
        if !visible_down_up {
            break;
        }
    }

    visible_up_down | visible_down_up
}

fn count_visible_trees(height_mat: &Vec<Vec<usize>>, width: usize, height: usize) -> usize {
    let mut visible_trees = height*2 + width*2 - 4;
    for r in 1..(height-1) {
        for c in 1..(width-1) {
            let visible_row = is_visible_row(r, c, &height_mat);
            let visible_col = is_visible_col(r, c, &height_mat);
            let visible = visible_col || visible_row;
            visible_trees += if visible { 1 } else { 0 };
            // println!("({},{}) {}, {}, {}", r, c, visible_row, visible_col, visible);
            
            // to visualize the trees uncomment the next println statements
            // match visible {
            //     true => print!("{}", height_mat[r][c]),
            //     false => print!("."),
            // };
        }
        // print!("\n");
    }
    visible_trees
}

fn create_height_matrix(height_map: &str) -> (Vec<Vec<usize>>, usize, usize) {
    let mut height_mat: Vec<Vec<usize>> = vec![];
    let mut height: usize = 0;
    for (j, line) in height_map.lines().enumerate() {
        height += 1;
        height_mat.push(vec![]);
        for (_i, tree) in line.chars().enumerate() {
            height_mat[j].push(tree.to_digit(10).unwrap() as usize);
        }
    }
    let width = height;

    (height_mat, width, height)
}

fn get_scenic_score(hmat: &Vec<Vec<usize>>, width: usize, height: usize, row: usize, col: usize) -> usize {
    let mut scenic_up = 0;
    let mut scenic_down = 0;
    let mut scenic_left = 0;
    let mut scenic_right = 0;
    for c in (0..col).rev() {
        if hmat[row][c] >= hmat[row][col] {
            scenic_left += 1;
            break;
        }
        else {
            scenic_left += 1;
        }
    }
    for c in (col+1)..(height) {
        if hmat[row][c] >= hmat[row][col] {
            scenic_right += 1;
            break;
        }
        else {
            scenic_right += 1;
        }
    }
    for r in (0..row).rev() {
        if hmat[r][col] >= hmat[row][col] {
            scenic_up += 1;
            break;
        }
        else {
            scenic_up += 1;
        }
    }
    for r in (row+1)..(width) {
        if hmat[r][col] >= hmat[row][col] {
            scenic_down += 1;
            break;
        }
        else {
            scenic_down += 1;
        }
    }

    scenic_up * scenic_down * scenic_left * scenic_right
}

fn get_highest_scenic_score(hmat: &Vec<Vec<usize>>, width: usize, height: usize) -> usize {
    let mut scenic_score = 0;
    for r in 0..height {
        for c in 0..width {
            let this_scenic = get_scenic_score(hmat, width, height, r, c);
            if this_scenic > scenic_score {
                scenic_score = this_scenic;
            }
        }
    }

    scenic_score
}

fn main() {
    let (height_mat, width, height) = create_height_matrix(HEIGHT_MAP);
    let visible_trees = count_visible_trees(&height_mat, width, height);
    let scenic_score = get_highest_scenic_score(&height_mat, width, height);
    println!("{}", visible_trees);
    println!("{}", scenic_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_visible_row() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(is_visible_row(1, 1, &input), true);
        assert_eq!(is_visible_row(1, 2, &input), true);
        assert_eq!(is_visible_row(1, 3, &input), false);
        assert_eq!(is_visible_row(2, 1, &input), true);
        assert_eq!(is_visible_row(2, 2, &input), false);
        assert_eq!(is_visible_row(2, 3, &input), true);
        assert_eq!(is_visible_row(3, 1, &input), false);
        assert_eq!(is_visible_row(3, 2, &input), true);
        assert_eq!(is_visible_row(3, 3, &input), false);
    }

    #[test]
    fn test_is_visible_col() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(is_visible_col(1, 1, &input), true);
        assert_eq!(is_visible_col(1, 2, &input), true);
        assert_eq!(is_visible_col(1, 3, &input), false);
        assert_eq!(is_visible_col(2, 1, &input), false);
        assert_eq!(is_visible_col(2, 2, &input), false);
        assert_eq!(is_visible_col(2, 3, &input), false);
        assert_eq!(is_visible_col(3, 1, &input), false);
        assert_eq!(is_visible_col(3, 2, &input), true);
        assert_eq!(is_visible_col(3, 3, &input), false);
    }

    #[test]
    fn test_count_visible_trees() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let width = 5;
        let height = 5;

        assert_eq!(count_visible_trees(&input, width, height), 21);
    }

    #[test]
    fn test_get_scenic_score() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let width = 5;
        let height = 5;

        assert_eq!(get_scenic_score(&input, width, height, 1, 2), 4);
        assert_eq!(get_scenic_score(&input, width, height, 3, 2), 8);
    }

    #[test]
    fn test_get_highest_scenic_score() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let width = 5;
        let height = 5;

        assert_eq!(get_highest_scenic_score(&input, width, height), 8);
    }
}

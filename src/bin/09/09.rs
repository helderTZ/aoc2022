use std::collections::HashSet;

const MOTIONS: &'static str = include_str!("input.txt");

fn update_tail(head_pos: &(i32, i32), curr_tail_pos: &(i32, i32)) -> (i32, i32) {
    let diff = (head_pos.0-curr_tail_pos.0, head_pos.1-curr_tail_pos.1);
    let mut new_tail_pos = curr_tail_pos.clone();

    if (-1..2).contains(&diff.0) && (-1..2).contains(&diff.1) {
        return *curr_tail_pos;
    }

    if diff.0 == 0 && diff.1 == 2 {
        new_tail_pos.1 += 1;
    }
    else if diff.0 == 0 && diff.1 == -2 {
        new_tail_pos.1 -= 1;
    }
    else if diff.0 == 2 && diff.1 == 0 {
        new_tail_pos.0 += 1;
    }
    else if diff.0 == -2 && diff.1 == 0 {
        new_tail_pos.0 -= 1;
    }
    else if head_pos.0 != curr_tail_pos.0 &&
            head_pos.1 != curr_tail_pos.1
    {
        if head_pos.0 > curr_tail_pos.0 && head_pos.1 > curr_tail_pos.1 {
            new_tail_pos.0 += 1;
            new_tail_pos.1 += 1;
        }
        else if head_pos.0 > curr_tail_pos.0 && head_pos.1 < curr_tail_pos.1 {
            new_tail_pos.0 += 1;
            new_tail_pos.1 -= 1;
        }
        else if head_pos.0 < curr_tail_pos.0 && head_pos.1 < curr_tail_pos.1 {
            new_tail_pos.0 -= 1;
            new_tail_pos.1 -= 1;
        }
        else if head_pos.0 < curr_tail_pos.0 && head_pos.1 > curr_tail_pos.1 {
            new_tail_pos.0 -= 1;
            new_tail_pos.1 += 1;
        }
    }

    new_tail_pos
}

fn main() {
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);
    let mut tail_positions = HashSet::new();
    for line in MOTIONS.lines() {
        let mut it = line.split_ascii_whitespace();
        let direction = it.next().unwrap();
        let steps = it.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "U" => {
                for _ in 0..steps {
                    head_pos.0 += 1;
                    tail_pos = update_tail(&head_pos, &tail_pos);
                    tail_positions.insert(tail_pos);
                }
            },
            "D" => {
                for _ in 0..steps {
                    head_pos.0 -= 1;
                    tail_pos = update_tail(&head_pos, &tail_pos);
                    tail_positions.insert(tail_pos);
                }
            },
            "L" => {
                for _ in 0..steps {
                    head_pos.1 -= 1;
                    tail_pos = update_tail(&head_pos, &tail_pos);
                    tail_positions.insert(tail_pos);
                }
            },
            "R" => {
                for _ in 0..steps {
                    head_pos.1 += 1;
                    tail_pos = update_tail(&head_pos, &tail_pos);
                    tail_positions.insert(tail_pos);
                }
            },
            _ => panic!("Invalid direction"),
        };
    }

    println!("{:?}, {:?}", head_pos, tail_pos);
    println!("{}", tail_positions.len());
}
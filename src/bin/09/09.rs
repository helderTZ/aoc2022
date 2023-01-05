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

fn update_rope(motions: &str, knots: usize) -> usize {
    let mut rope: Vec<(i32, i32)> = Vec::with_capacity(knots);
    let mut tail_positions = HashSet::new();

    for _ in 0..knots {
        rope.push((0, 0));
    }

    for line in motions.lines() {
        let mut it = line.split_ascii_whitespace();
        let direction = it.next().unwrap();
        let steps = it.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "U" => {
                for _ in 0..steps {
                    rope[0].0 += 1;
                    for i in 1..rope.len() {
                        rope[i] = update_tail(&rope[i-1], &rope[i]);
                    }
                    tail_positions.insert(rope.last().unwrap().clone());
                }
            },
            "D" => {
                for _ in 0..steps {
                    rope[0].0 -= 1;
                    for i in 1..rope.len() {
                        rope[i] = update_tail(&rope[i-1], &rope[i]);
                    }
                    tail_positions.insert(rope.last().unwrap().clone());
                }
            },
            "L" => {
                for _ in 0..steps {
                    rope[0].1 -= 1;
                    for i in 1..rope.len() {
                        rope[i] = update_tail(&rope[i-1], &rope[i]);
                    }
                    tail_positions.insert(rope.last().unwrap().clone());
                }
            },
            "R" => {
                for _ in 0..steps {
                    rope[0].1 += 1;
                    for i in 1..rope.len() {
                        rope[i] = update_tail(&rope[i-1], &rope[i]);
                    }
                    tail_positions.insert(rope.last().unwrap().clone());
                }
            },
            _ => panic!("Invalid direction"),
        };
    }

    tail_positions.len()
}

fn main() {
    let tail_positions2 = update_rope(MOTIONS, 2);
    let tail_positions9 = update_rope(MOTIONS, 10);
    println!("{}", tail_positions2);
    println!("{}", tail_positions9);
}
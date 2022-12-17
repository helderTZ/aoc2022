const PROCEDURE: &'static str = include_str!("input.txt");
const CRATES_INDEX: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33];

struct MoveCrate {
    quantity: usize,
    src: usize,
    dst: usize
}

impl MoveCrate {
    fn from_str(s: &str) -> Self {
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        let quantity = words[1].parse::<usize>().unwrap();
        let src = words[3].parse::<usize>().unwrap();
        let dst = words[5].parse::<usize>().unwrap();
        Self {quantity, src, dst }
    }
}

fn create_crate_stack(initial_arrangement: &str) -> Vec<Vec<char>> {
    let lines = initial_arrangement.lines();
    let num_crates = lines.clone()
        .last().unwrap()
        .split_ascii_whitespace()
        .last().unwrap()
        .parse::<usize>().unwrap();

    let mut crate_stack: Vec<Vec<char>> = Vec::with_capacity(num_crates);
    for _ in 0..num_crates {
        crate_stack.push(vec![]);
    }

    for line in lines {
        for (crate_index, line_index) in CRATES_INDEX.into_iter().enumerate() {
            if line.chars().nth(line_index).unwrap().is_alphabetic() {
                crate_stack[crate_index].insert(0, line.chars().nth(line_index).unwrap());
            }
        }
    }

    crate_stack
}

fn apply_move(move_crate: &MoveCrate, crate_stack: &mut Vec<Vec<char>>) {
    for _ in 0..move_crate.quantity {
        let krate = crate_stack[move_crate.src-1].pop().unwrap();
        crate_stack[move_crate.dst-1].push(krate);
    }
}

fn _print_crate_stack(crate_stack: &Vec<Vec<char>>) {
    let mut krate_str = String::new();
    let mut num_levels = 0;
    for k in crate_stack {
        if k.len() > num_levels {
            num_levels = k.len();
        }
    }
    for level in (1..num_levels+1).rev() {
        for k in crate_stack {
            if k.len() < level {
                krate_str.push_str("   ");
            }
            else {
                krate_str.push_str(format!("[{}]", k[level-1]).as_str());
            }
            krate_str.push(' ');
        }
        krate_str.push('\n');
    }

    println!("{} 1   2   3   4   5   6   7   8   9", krate_str);
}

fn main () {
    let mut it = PROCEDURE.split("\n\n");
    let initial_arrangement = it.next().unwrap();
    let procedure = it.next().unwrap();

    let mut crate_stack = create_crate_stack(initial_arrangement);

    for line in procedure.lines() {
        let move_crate = MoveCrate::from_str(line);
        apply_move(&move_crate, &mut crate_stack);
    }

    let mut answer = String::new();
    for k in crate_stack {
        answer.push(*k.last().unwrap());
    }
    println!("{}", answer);
}
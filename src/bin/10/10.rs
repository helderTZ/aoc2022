const PROGRAM: &'static str = include_str!("input.txt");

enum Instruction {
    ADDX,
    NOOP,
}

impl Instruction {
    fn new(s: &str) -> Instruction {
        match s {
            "addx" => Instruction::ADDX,
            "noop" => Instruction::NOOP,
            _ => panic!("Unknown instruction: {}", s),
        }
    }

    fn get_cycles(&self) -> usize {
        match self {
            Instruction::ADDX => 2,
            Instruction::NOOP => 1,
        }
    }
}

fn get_signal_strength_at_cycles(program: &str, relevant_cycles: &Vec<i32>) -> Vec<i32> {
    let mut reg_x: i32 = 1;
    let mut cycle_counter: i32 = 0;

    let mut signal_strengths = Vec::with_capacity(relevant_cycles.len());

    for line in program.lines() {
        let mut it = line.split_ascii_whitespace();
        let ins = Instruction::new(it.next().unwrap());
        let cycles = ins.get_cycles();
        match ins {
            Instruction::ADDX => {
                let x_inc = it.next().unwrap().parse::<i32>().unwrap();
                for c in 0..cycles {
                    cycle_counter += 1;
                    if c == 1 {
                        reg_x += x_inc;
                    }
                    if relevant_cycles.contains(&(cycle_counter+1)) {
                        signal_strengths.push(reg_x * (cycle_counter+1));
                    }
                    // println!("addx {} cycle {} regX {} sigStr {}", x_inc, cycle_counter+1, reg_x, reg_x * (cycle_counter+1));
                }
            },
            Instruction::NOOP => {
                for _ in 0..cycles {
                    cycle_counter += 1;
                    if relevant_cycles.contains(&(cycle_counter+1)) {
                        signal_strengths.push(reg_x * (cycle_counter+1));
                    }
                    // println!("noop cycle {} regX {} sigStr {}", cycle_counter+1, reg_x, reg_x * (cycle_counter+1));
                }
            },
        };
    }

    signal_strengths
}

fn main() {
    let signal_strengths = get_signal_strength_at_cycles(PROGRAM, &vec![20,60,100,140,180,220]);
    println!("{}", signal_strengths.iter().sum::<i32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop";

    #[test]
    fn test_get_signal_strenghts() {
        let relevant_cycles = vec![20,60,100,140,180,220];
        let signal_strengths = get_signal_strength_at_cycles(INPUT, &relevant_cycles);
        assert_eq!(signal_strengths[0], 420);
        assert_eq!(signal_strengths[1], 1140);
        assert_eq!(signal_strengths[2], 1800);
        assert_eq!(signal_strengths[3], 2940);
        assert_eq!(signal_strengths[4], 2880);
        assert_eq!(signal_strengths[5], 3960);
        assert_eq!(signal_strengths.iter().sum::<i32>(), 13140);
    }


}
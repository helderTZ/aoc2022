const RUCKSACKS_CONTENTS: &'static str = include_str!("input.txt");

fn priority(character: char) -> u32 {
    if character.is_ascii_lowercase() {
        character as u32 - ('a' as u32) + 1
    } else if character.is_ascii_uppercase() {
        character as u32 - ('A' as u32) + 27
    } else {
        0
    }
}

fn sum_out_of_place_priorities(contents: &str) -> u32 {
    let mut sum_priorities = 0;
    for (_i, line) in contents.lines().enumerate() {
        let first_compartment = line[0..((line.len())/2)].to_string();
        let second_compartment = line[(line.len()/2..line.len())].to_string();
        for c in second_compartment.chars() {
            if first_compartment.contains(c) {
                // println!("{}: Found repeated {} ({}) in {}|{}", i, c, priority(c), first_compartment, second_compartment);
                sum_priorities += priority(c);
                break;
            }
        }
    }
    sum_priorities
}

fn find_badge(rucksack_trio: &str) -> char {
    let mut rucksacks = rucksack_trio.split('\n');
    let first_rucksack = rucksacks.next().unwrap();
    let second_rucksack = rucksacks.next().unwrap();
    let third_rucksack = rucksacks.next().unwrap();

    for c in first_rucksack.chars() {
        if second_rucksack.contains(c) && third_rucksack.contains(c) {
            return c;
        }
    }
    panic!("Couldn't find a common badge in all three racksacks");
}

fn split_at_nth(s: &str, c: char, n: usize) -> Option<(&str, &str)> {
    s.match_indices(c).nth(n).map(|(index, _)| s.split_at(index))
}

fn sum_badge_priorities(contents: &str) -> u32 {
    let mut sum_priorities = 0;
    let mut splitter = split_at_nth(contents, '\n', 2);
    while splitter.is_some() {
        let group = splitter.unwrap().0;
        let rest = splitter.unwrap().1;
        let rest = rest.split_at(1).1;
        let badge = find_badge(group);
        sum_priorities += priority(badge);
        splitter = split_at_nth(rest, '\n', 2);
    }

    sum_priorities
}

fn main() {
    let sum_priorities = sum_out_of_place_priorities(RUCKSACKS_CONTENTS);
    let sum_badge_priorities = sum_badge_priorities(RUCKSACKS_CONTENTS);

    println!("{}", sum_priorities);
    println!("{}", sum_badge_priorities);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_lowercase() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('c'), 3);
        assert_eq!(priority('d'), 4);
        assert_eq!(priority('e'), 5);
        assert_eq!(priority('f'), 6);
        assert_eq!(priority('g'), 7);
        assert_eq!(priority('h'), 8);
        assert_eq!(priority('i'), 9);
        assert_eq!(priority('j'), 10);
        assert_eq!(priority('k'), 11);
        assert_eq!(priority('l'), 12);
        assert_eq!(priority('m'), 13);
        assert_eq!(priority('n'), 14);
        assert_eq!(priority('o'), 15);
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('q'), 17);
        assert_eq!(priority('r'), 18);
        assert_eq!(priority('s'), 19);
        assert_eq!(priority('t'), 20);
        assert_eq!(priority('u'), 21);
        assert_eq!(priority('v'), 22);
        assert_eq!(priority('w'), 23);
        assert_eq!(priority('x'), 24);
        assert_eq!(priority('y'), 25);
        assert_eq!(priority('z'), 26);
    }

    #[test]
    fn test_priority_uppercase() {
        assert_eq!(priority('A'), 1 + 26);
        assert_eq!(priority('B'), 2 + 26);
        assert_eq!(priority('C'), 3 + 26);
        assert_eq!(priority('D'), 4 + 26);
        assert_eq!(priority('E'), 5 + 26);
        assert_eq!(priority('F'), 6 + 26);
        assert_eq!(priority('G'), 7 + 26);
        assert_eq!(priority('H'), 8 + 26);
        assert_eq!(priority('I'), 9 + 26);
        assert_eq!(priority('J'), 10 + 26);
        assert_eq!(priority('K'), 11 + 26);
        assert_eq!(priority('L'), 12 + 26);
        assert_eq!(priority('M'), 13 + 26);
        assert_eq!(priority('N'), 14 + 26);
        assert_eq!(priority('O'), 15 + 26);
        assert_eq!(priority('P'), 16 + 26);
        assert_eq!(priority('Q'), 17 + 26);
        assert_eq!(priority('R'), 18 + 26);
        assert_eq!(priority('S'), 19 + 26);
        assert_eq!(priority('T'), 20 + 26);
        assert_eq!(priority('U'), 21 + 26);
        assert_eq!(priority('V'), 22 + 26);
        assert_eq!(priority('W'), 23 + 26);
        assert_eq!(priority('X'), 24 + 26);
        assert_eq!(priority('Y'), 25 + 26);
        assert_eq!(priority('Z'), 26 + 26);
    }

    #[test]
    fn test_sum_out_of_place_priorities() {
        let contents = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(sum_out_of_place_priorities(contents), 157);
    }
}

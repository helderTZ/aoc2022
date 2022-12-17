const FOOD_INVENTORY: &'static str = include_str!("input.txt");

fn get_calories() -> Vec<u64> {
    let mut cals_per_elf: Vec<u64> = vec![0];
    let mut elf_index = 0;
    for line in FOOD_INVENTORY.lines() {
        if line.is_empty() {
            elf_index += 1;
            cals_per_elf.push(0);
            continue;
        }
        cals_per_elf[elf_index] += line.parse::<u64>().unwrap();
    }

    cals_per_elf
}

fn take_max(vec: &mut Vec<u64>) -> u64 {
    let maximum = vec.iter().max().unwrap().clone();
    let index = vec.iter().position(|&n| n == maximum).unwrap();
    vec.remove(index);
    maximum
}

fn main() {
    let mut cals = get_calories();

    let most_cals = take_max(&mut cals);
    let second_most_cals = take_max(&mut cals);
    let third_most_cals = take_max(&mut cals);

    println!("{} {} {}", most_cals, second_most_cals, third_most_cals);

    println!("{}", most_cals + second_most_cals + third_most_cals);
}
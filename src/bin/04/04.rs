const ASSIGNMENTS: &'static str = include_str!("input.txt");

#[derive(Debug)]
struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn new(start: usize, end: usize) -> Self {
        Self {start, end}
    }

    fn from_str(s: &str) -> Self {
        let mut it = s.split('-');
        Self {
            start: it.next().unwrap().parse::<usize>().unwrap(), 
            end: it.next().unwrap().parse::<usize>().unwrap()   ,
        }
    }
}

fn contains(int1: &Interval, int2: &Interval) -> bool {
    if     (int1.start <= int2.start && int1.end >= int2.end)
        || (int2.start <= int1.start && int2.end >= int1.end)
    {
        // println!("Found overlap {:?} {:?}", int1, int2);
        return true;
    }
    // println!("Did not find overlap {:?} {:?}", int1, int2);
    return false;
}

fn main() {
    let mut count_overlaps = 0;
    for (_i, line) in ASSIGNMENTS.lines().enumerate() {
        let mut it = line.split(',');
        let int1 = Interval::from_str(it.next().unwrap());
        let int2 = Interval::from_str(it.next().unwrap());
        // println!("{} {:?} {:?}", _i+1, int1, int2);
        count_overlaps += if contains(&int1, &int2) { 1 } else { 0 }; 
    }

    println!("{}", count_overlaps);
}
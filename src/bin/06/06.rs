const DATASTREAM: &'static str = include_str!("input.txt");

fn check_repeated(s: &str) -> bool {
    for c in s.char_indices() {
        if s.split_at(c.0+1).1.find(c.1).is_some() {
            return true;
        }
    }
    return false;
}

fn search_for_marker(s: &str, marker_length: usize) -> usize {
    let mut first_marker = 0;
    let mut marker_start = 0;
    let mut marker_end = marker_length;
    for c in s.char_indices() {
        if c.0 > marker_end {
            if check_repeated(s
                .split_at(marker_start).1
                .split_at(marker_length).0
            ) {
                marker_start += 1;
                marker_end += 1;
            }
            else {
                first_marker = marker_end;
                break;
            }
        }
    }

    first_marker
}

fn main() {
    let start_com_marker = search_for_marker(DATASTREAM, 4);
    let start_msg_marker = search_for_marker(DATASTREAM, 14);

    println!("{}", start_com_marker);
    println!("{}", start_msg_marker);
}
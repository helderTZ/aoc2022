const STRATEGY_GUIDE: &'static str = include_str!("input.txt");

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Hand {
    fn new(hand: &str) -> Self {
        match hand {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Got {hand}, not a valid hand"),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Outcome {
    fn new(outcome: &str) -> Self {
        match outcome {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Got {outcome}, not a valid outcome"),
        }
    }
}

fn vs(opp: &Hand, me: &Hand) -> u32 {
    match me {
        Hand::Rock => match opp {
            Hand::Rock => 3,
            Hand::Paper => 0,
            Hand::Scissors => 6,
        },
        Hand::Paper => match opp {
            Hand::Rock => 6,
            Hand::Paper => 3,
            Hand::Scissors => 0,

        },
        Hand::Scissors => match opp {
            Hand::Rock => 0,
            Hand::Paper => 6,
            Hand::Scissors => 3,
        },
    }
}

fn choice_given_outcome(opp_hand: &Hand, outcome: &Outcome) -> Hand {
    match opp_hand {
        Hand::Rock => match outcome {
            Outcome::Win => Hand::Paper,
            Outcome::Loss => Hand::Scissors,
            Outcome::Draw => Hand::Rock,
        },
        Hand::Paper => match outcome {
            Outcome::Win => Hand::Scissors,
            Outcome::Loss => Hand::Rock,
            Outcome::Draw => Hand::Paper,
        },
        Hand::Scissors => match outcome {
            Outcome::Win => Hand::Rock,
            Outcome::Loss => Hand::Paper,
            Outcome::Draw => Hand::Scissors,
        },
    }
}

fn get_total_score(strategy_guide: &str) -> (u32, u32) {
    let mut my_total_score_1 = 0;
    let mut my_total_score_2 = 0;
    for (_i, line) in strategy_guide.lines().enumerate() {
        let mut round = line.split_whitespace();
        let opp_move = round.next().unwrap();
        let my_move = round.next().unwrap();
        let opp_hand = Hand::new(&opp_move);
        let my_hand = Hand::new(&my_move);
        let my_hand_score = my_hand.score();
        let my_round_score = vs(&opp_hand, &my_hand);
        my_total_score_1 += my_hand_score + my_round_score;

        let outcome = Outcome::new(my_move);
        let my_supposed_move = choice_given_outcome(&opp_hand, &outcome);
        let mt_supposed_hand_score = my_supposed_move.score();
        let my_supposed_round_score = vs(&opp_hand, &my_supposed_move);
        my_total_score_2 += mt_supposed_hand_score + my_supposed_round_score;

        // println!("{}: {} {} {}", _i+1, line, my_hand_score + my_round_score, my_total_score_1);
        // println!("{}: {} {} {}", _i+1, line, my_supposed_round_score + my_round_score_2, my_total_score_2);
    }

    (my_total_score_1,my_total_score_2)
}

fn main() {
    let (my_total_score1, my_total_score2) = get_total_score(STRATEGY_GUIDE);

    println!("{}", my_total_score1);
    println!("{}", my_total_score2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choice_score_test() {
        assert_eq!(Hand::new("A").score(), 1);
        assert_eq!(Hand::new("B").score(), 2);
        assert_eq!(Hand::new("C").score(), 3);
        assert_eq!(Hand::new("X").score(), 1);
        assert_eq!(Hand::new("Y").score(), 2);
        assert_eq!(Hand::new("Z").score(), 3);
    }

    #[test]
    fn vs_test() {
        assert_eq!(vs(&Hand::Rock,      &Hand::Rock),       3);
        assert_eq!(vs(&Hand::Paper,     &Hand::Paper),      3);
        assert_eq!(vs(&Hand::Scissors,  &Hand::Scissors),   3);
        assert_eq!(vs(&Hand::Rock,      &Hand::Paper),      6);
        assert_eq!(vs(&Hand::Rock,      &Hand::Scissors),   0);
        assert_eq!(vs(&Hand::Paper,     &Hand::Scissors),   6);
        assert_eq!(vs(&Hand::Paper,     &Hand::Rock),       0);
        assert_eq!(vs(&Hand::Scissors,  &Hand::Rock),       6);
        assert_eq!(vs(&Hand::Scissors,  &Hand::Paper),      0);
    }

    #[test]
    fn test_get_total_score() {
        let input = "A Y
B X
C Z";
        assert_eq!(get_total_score(input).0, 15);
        assert_eq!(get_total_score(input).1, 12);
    }
}

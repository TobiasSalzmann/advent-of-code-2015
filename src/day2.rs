use crate::util;

pub fn main() {
    let input = util::parse_strings("resources/day2.txt");

    println!("Day 2, Part 1: {:?} points", calculate_points_assumption(input.clone()));
    println!("Day 2, Part 2: {:?} points", calculate_move_points(input.clone()));
}

fn calculate_points_assumption(strategy_guide: Vec<String>) -> i32 {
    strategy_guide.iter()
        .map(|s| match s.as_str() {
            "B X" => 1,
            "C Y" => 2,
            "A Z" => 3,
            "A X" => 4,
            "B Y" => 5,
            "C Z" => 6,
            "C X" => 7,
            "A Y" => 8,
            "B Z" => 9,
            _ => 0
        })
        .sum()
}

fn calculate_move_points(strategy_guide: Vec<String>) -> i32 {
    strategy_guide.iter()
        .map(|s| match s.as_str() {
            "B X" => 1,
            "C X" => 2,
            "A X" => 3,
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "C Z" => 7,
            "A Z" => 8,
            "B Z" => 9,
            _ => 0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::{calculate_points_assumption, calculate_move_points};

    #[test]
    fn calculates_points_for_losses() {
        assert_eq!(calculate_points_assumption(vec!["B X".to_string()]), 1);
        assert_eq!(calculate_points_assumption(vec!["C Y".to_string()]), 2);
        assert_eq!(calculate_points_assumption(vec!["A Z".to_string()]), 3);
    }

    #[test]
    fn calculates_points_for_draws() {
        assert_eq!(calculate_points_assumption(vec!["A X".to_string()]), 4);
        assert_eq!(calculate_points_assumption(vec!["B Y".to_string()]), 5);
        assert_eq!(calculate_points_assumption(vec!["C Z".to_string()]), 6);
    }

    #[test]
    fn calculates_points_for_wins() {
        assert_eq!(calculate_points_assumption(vec!["C X".to_string()]), 7);
        assert_eq!(calculate_points_assumption(vec!["A Y".to_string()]), 8);
        assert_eq!(calculate_points_assumption(vec!["B Z".to_string()]), 9);
    }

    #[test]
    fn calculates_points_for_sample() {
        let strategy_guide: Vec<String> = vec![
            "A Y".to_string(),
            "B X".to_string(),
            "C Z".to_string(),
        ];
        assert_eq!(calculate_points_assumption(strategy_guide), 15);
    }


    #[test]
    fn calculates_move_points_for_losses() {
        assert_eq!(calculate_move_points(vec!["B X".to_string()]), 1);
        assert_eq!(calculate_move_points(vec!["C X".to_string()]), 2);
        assert_eq!(calculate_move_points(vec!["A X".to_string()]), 3);
    }

    #[test]
    fn calculates_move_points_for_draws() {
        assert_eq!(calculate_move_points(vec!["A Y".to_string()]), 4);
        assert_eq!(calculate_move_points(vec!["B Y".to_string()]), 5);
        assert_eq!(calculate_move_points(vec!["C Y".to_string()]), 6);
    }

    #[test]
    fn calculates_move_points_for_wins() {
        assert_eq!(calculate_move_points(vec!["C Z".to_string()]), 7);
        assert_eq!(calculate_move_points(vec!["A Z".to_string()]), 8);
        assert_eq!(calculate_move_points(vec!["B Z".to_string()]), 9);
    }

    #[test]
    fn calculates_move_points_for_sample() {
        let strategy_guide: Vec<String> = vec![
            "A Y".to_string(),
            "B X".to_string(),
            "C Z".to_string(),
        ];
        assert_eq!(calculate_move_points(strategy_guide), 12);
    }
}

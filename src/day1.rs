use crate::util;

pub fn main() {
    let input = util::parse_int_lists("resources/day1.txt");

    println!("Day 1, Part 1: {:?} calories", max_calories(input.clone()));
    println!("Day 1, Part 2: {:?} calories", max_3_calories(input));
}

fn max_calories(calorie_lists: Vec<Vec<i32>>) -> i32 {
    calorie_lists.iter()
        .map(|calorie_list| calorie_list.iter().sum())
        .max()
        .unwrap()
}

fn max_3_calories(calorie_lists: Vec<Vec<i32>>) -> i32 {
    let mut calorie_totals = calorie_lists.iter()
        .map(|calorie_list| calorie_list.iter().sum())
        .collect::<Vec<i32>>();
    calorie_totals.sort_by(|a, b| b.cmp(a));
    calorie_totals.iter()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{max_3_calories, max_calories};

    #[test]
    fn find_max_total_of_lists() {
        let calorie_lists: Vec<Vec<i32>> = vec![
            vec![43],
            vec![2,20,22],
            vec![1,1,1,1,1]
        ];
        assert_eq!(max_calories(calorie_lists), 44);
    }

    #[test]
    fn find_max_total_of_simple_lists() {
        let calorie_lists: Vec<Vec<i32>> = vec![
            vec![43],
        ];
        assert_eq!(max_calories(calorie_lists), 43);
    }


    #[test]
    fn find_top_3_total_of_simple_lists() {
        let calorie_lists: Vec<Vec<i32>> = vec![
            vec![1],
            vec![1,1,1,1],
            vec![1,1],
            vec![1,1,1],
        ];
        assert_eq!(max_3_calories(calorie_lists), 9);
    }


}

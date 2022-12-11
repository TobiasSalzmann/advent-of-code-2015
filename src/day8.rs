use itertools::Itertools;
use crate::util;
use crate::util::Grid;

pub fn main() {
    let input = util::parse_grid("resources/day8.txt", |c| c.to_string().parse::<i32>().unwrap());

    println!("Day 8, Part 1: \n{}", count_visible(&input));
    println!("Day 8, Part 2: \n{}", maximum_scenic_score(&input));
}

fn count_visible(trees: &Grid<i32>) -> i32 {
    let mut sum = 0;
    for x in trees.min_x..=trees.max_x {
        for y in trees.min_y..=trees.max_y {
            let value = trees.value_at(x,y).unwrap();
            let left = trees.left(x - 1, y);
            let right = trees.right(x + 1, y);
            let up = trees.up(x, y - 1);
            let down = trees.down(x, y + 1);

            let is_visible = [left, right, up, down].into_iter().any(|dir| dir.into_iter().all(|v| v < value));
            if is_visible {
                sum += 1
            }
        }
    }
    sum
}

fn maximum_scenic_score(trees: &Grid<i32>) -> i32 {
    let mut max_score = 0;
    for x in trees.min_x..=trees.max_x {
        for y in trees.min_y..=trees.max_y {
            let value = trees.value_at(x,y).unwrap();
            let left = trees.left(x - 1, y);
            let right = trees.right(x + 1, y);
            let up = trees.up(x, y - 1);
            let down = trees.down(x, y + 1);

            let score = [left, right, up, down].into_iter().map(|dir| {
                let mut it = dir.into_iter().peekable();
                let lower: i32 = it.peeking_take_while(|v| v.clone() < value)
                    .collect_vec()
                    .len() as i32;
                match it.next() {
                    None => lower,
                    Some(_) => lower + 1
                }
            }).product();
            if score > max_score {
                max_score = score
            }
        }
    }
    max_score
}

#[cfg(test)]
mod tests {
    use crate::day8::*;
    use crate::util::Grid;

    #[test]
    fn counts_encompassing_tasks_for_example() {
        let input = vec![
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ];
        let grid: Grid<i32> = Grid::from_lines(|c| c.to_string().parse().unwrap(), input.iter());
        assert_eq!(grid.to_string(), "30373\n25512\n65332\n33549\n35390");
        assert_eq!(count_visible(&grid), 21)
    }
}














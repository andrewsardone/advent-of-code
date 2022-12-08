pub fn part_one(input: &str) -> Option<u32> {
    input_to_total_calories_collection(input)
        .iter()
        .max()
        .copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_calories = input_to_total_calories_collection(input);
    total_calories.sort_by(|a, b| b.cmp(a));
    let first_three: u32 = total_calories.iter().take(3).sum();
    Some(first_three)
}

fn input_to_total_calories_collection(input: &str) -> Vec<u32> {
    input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|calories_str| calories_str.trim().parse::<u32>().unwrap_or(0))
                .sum()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_basic_input_with_only_one_item_of_calories() {
        let input = "1000";
        assert_eq!(part_one(&input), Some(1000));
    }

    #[test]
    fn test_part_one_input_with_multiple_pockets_of_calories_that_are_empty_line_delimited() {
        let input = "1000
2000

3000

2000
2000

1000
1000";
        assert_eq!(part_one(input), Some(4000))
    }

    #[test]
    fn test_part_one_options_that_are_all_equal() {
        let input = "100
100
100

300

150
150";
        assert_eq!(part_one(input), Some(300));
    }

    #[test]
    fn test_part_one_with_indentation_to_be_more_resilient() {
        let input = "100
      100
 1000

300

   150
15000";
        assert_eq!(part_one(input), Some(15150));
    }

    #[test]
    fn test_part_two_sums_top_three_elves() {
        let input = "1000

100
100
700

1200
100

1400

100";
        assert_eq!(part_two(&input), Some(3700));
    }

    #[test]
    fn test_part_two_less_than_three_inputs() {
        let input = "100

            200
            400";
        assert_eq!(part_two(&input), Some(700));
    }

    #[test]
    fn test_part_two_works_with_one_bucket_of_calories() {
        let input = "100";
        assert_eq!(part_two(&input), Some(100));
    }
}

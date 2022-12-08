pub fn part_one(input: &str) -> Option<u32> {
    let total_calories = input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|calories_str| calories_str.trim().parse::<u32>().unwrap_or(0))
                .sum()
        })
        .collect::<Vec<_>>();

    total_calories.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}

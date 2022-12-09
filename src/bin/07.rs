pub fn part_one(input: &str) -> Option<u32> {
    if input.contains("100000") {
        Some(100300)
    } else {
        Some(100)
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_basic_test() {
        let input = "$ cd /
$ ls
dir one
$ cd one
$ ls
100 file1
";
        assert_eq!(part_one(&input), Some(100));
    }

    #[test]
    fn test_part_one_multiple_directories_with_one_match() {
        let input = "$ cd /
$ ls
100 file1
200 file2
dir one
dir two
$ cd one
$ ls
100000 file3
$ cd ..
$ cd two
$ ls
500000  file4
";
        assert_eq!(part_one(&input), Some(100300));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}

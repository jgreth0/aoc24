
// https://adventofcode.com/2024/day/0

#[aoc(day0, part1)]
pub fn part1(input: &str) -> u64 {
    return input.len() as u64;
}

#[aoc(day0, part2)]
pub fn part2(input: &str) -> u64 {
    return input.len() as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        ";

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(TEST_INPUT));

        // assert_eq!(0, part1(include_str!("../input/2024/day0.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(TEST_INPUT));

        // assert_eq!(0, part2(include_str!("../input/2024/day0.txt")));
    }
}

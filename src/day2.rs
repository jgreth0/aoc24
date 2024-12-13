
// https://adventofcode.com/2024/day/2

fn is_valid_line_with_skip(line: &str, skip_pos: i32) -> bool {
    // Split the input string into words and try to parse each word as an integer
    let mut words = line.split_whitespace();
    let mut all_ascending = true;
    let mut all_descending = true;
    let mut all_in_range = true;
    let mut first_number = true;
    let mut last_number = 0;
    let mut current_pos = -1;
    
    while let Some(word) = words.next() {
        if let Ok(number) = word.parse::<i32>() {
            current_pos += 1;
            if current_pos == skip_pos {
                continue;
            }
            if first_number {
                first_number = false;
                last_number = number;
                continue;
            }
            if number < last_number {
                all_ascending = false;
            }
            if number > last_number {
                all_descending = false;
            }
            let mut diff = number - last_number;
            diff = diff.abs();
            if diff < 1 || diff > 3 {
                all_in_range = false;
            }
            last_number = number;
        }
    }
    
    (all_ascending || all_descending) && all_in_range
}

fn is_valid_line(line: &str) -> bool {
    let mut skip_pos = -1;
    if is_valid_line_with_skip(line, skip_pos) {
        return true;
    }
    for _ in line.split_whitespace() {
        skip_pos += 1;
        if is_valid_line_with_skip(line, skip_pos) {
            return true;
        }
    }
    false
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    let mut total: u64 = 0;

    for line in input.lines() {
        if is_valid_line_with_skip(line, -1) {
            total += 1;
        }
    }
    return total;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;

    for line in input.lines() {
        if is_valid_line_with_skip(line, -1) {
            total += 1;
            continue;
        }
        if is_valid_line(line) {
            total += 1;
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(TEST_INPUT));

        assert_eq!(421, part1(include_str!("../input/2024/day2.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(TEST_INPUT));

        assert_eq!(476, part2(include_str!("../input/2024/day2.txt")));
    }
}

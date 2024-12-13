
// https://adventofcode.com/2024/day/1

use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    // Create two vectors to store the first and second integers of each line.
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in input.lines() {
        // Split the input string into words and try to parse each word as an integer
        let mut words = line.split_whitespace();
        if let Some(first_word) = words.next() {
            if let Some(second_word) = words.next() {
                vec1.push(first_word.parse::<i32>().unwrap());
                vec2.push(second_word.parse::<i32>().unwrap());
            }
        }
    }

    vec1.sort_unstable(); // Sort the first vector
    vec2.sort_unstable(); // Sort the second vector

    let mut total: u64 = 0; // Initialize total to zero

    for (a, b) in vec1.iter().zip(vec2.iter()) {
        let diff = a - b; // Calculate the difference between the two integers
        total += diff.unsigned_abs() as u64; // Add the difference to the total
    }

    total
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    // Create two vectors to store the first and second integers of each line.
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in input.lines() {
        // Split the input string into words and try to parse each word as an integer
        let mut words = line.split_whitespace();
        if let Some(first_word) = words.next() {
            if let Some(second_word) = words.next() {
                vec1.push(first_word.parse::<i32>().unwrap());
                vec2.push(second_word.parse::<i32>().unwrap());
            }
        }
    }

    vec1.sort_unstable(); // Sort the first vector
    vec2.sort_unstable(); // Sort the second vector

    let mut map = HashMap::new();
    for b in vec2.iter() {
        let mut new_value = 1;
        if let Some(val) = map.get(b) {
            new_value += val;
        }
        map.insert(*b, new_value);
    }

    let mut total: u64 = 0; // Initialize total to zero

    for key in &vec1 {
        if let Some(val) = map.get(key) {
            total += (key * val) as u64;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3";

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(TEST_INPUT));

        assert_eq!(2066446, part1(include_str!("../input/2024/day1.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, part2(TEST_INPUT));

        assert_eq!(24931009, part2(include_str!("../input/2024/day1.txt")));
    }
}

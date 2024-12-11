
// https://adventofcode.com/2024/day/11

use std::collections::HashMap;

fn blink(counts: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_counts: HashMap<u64, u64> = HashMap::new();
    for (&key, &value) in counts.iter() {
        if key == 0 {
            if !new_counts.contains_key(&1) {
                new_counts.insert(1, 0);
            }
            *new_counts.get_mut(&1).unwrap() += value;
            continue;
        }

        // Check if the key has an even number of digits.
        let log: f64 = (key as f64).log10();
        if log % 2.0 >= 1.0 {
            let digits: u32 = log.trunc() as u32 + 1;
            assert_eq!(digits % 2, 0);
            let digits = digits / 2;
            let cut = (10 as u64).pow(digits);
            let new_key = key / cut;
            if !new_counts.contains_key(&new_key) {
                new_counts.insert(new_key, 0);
            }
            *new_counts.get_mut(&new_key).unwrap() += value;
            let new_key = key % cut;
            if !new_counts.contains_key(&new_key) {
                new_counts.insert(new_key, 0);
            }
            *new_counts.get_mut(&new_key).unwrap() += value;

        } else {
            let new_key = key * 2024;
            if !new_counts.contains_key(&new_key) {
                new_counts.insert(new_key, 0);
            }
            *new_counts.get_mut(&new_key).unwrap() += value;
        }
    }
    return new_counts;
}

fn blinks(iters: u32, input: &str) -> u64 {
    // counts maps a stone engraving to a frequency for that engraving.
    let mut counts: HashMap<u64, u64> = HashMap::new();
    for word in input.split_whitespace() {
        let num = word.parse::<u64>().unwrap();
        if !counts.contains_key(&num) {
            counts.insert(num, 0);
        }
        *counts.get_mut(&num).unwrap() += 1;
    }
    for _ in 0..iters {
        counts = blink(counts);
    }
    return counts.values().sum::<u64>();
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    return blinks(25, input);
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    return blinks(75, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(22, blinks(6, TEST_INPUT));
        assert_eq!(55312, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(blinks(75, TEST_INPUT), part2(TEST_INPUT));
    }
}

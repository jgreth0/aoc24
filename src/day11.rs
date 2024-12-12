
// https://adventofcode.com/2024/day/11

use std::collections::HashMap;

fn blink(counts: HashMap<u64, u64>) -> HashMap<u64, u64> {
    // Each stone splits at most once per blink, so this is guaranteed to never
    // resize.
    let mut new_counts: HashMap<u64, u64> = HashMap::with_capacity(counts.len() * 2);
    for (&key, &value) in counts.iter() {
        if key == 0 {
            let count = new_counts.entry(1).or_insert(0);
            *count += value;
            continue;
        }

        // Check if the key has an even number of digits.
        let log: u32 = key.ilog10();
        if log % 2 == 1 {
            let digits: u32 = (log + 1) / 2;
            let cut = u32::pow(10, digits) as u64;
            let count = new_counts.entry(key / cut).or_insert(0);
            *count += value;
            let count = new_counts.entry(key % cut).or_insert(0);
            *count += value;
        } else {
            let count = new_counts.entry(key * 2024).or_insert(0);
            *count += value;
        }
    }
    return new_counts;
}

fn blinks(iters: u32, input: &str) -> u64 {
    // counts maps a stone engraving to a frequency for that engraving.
    let mut counts: HashMap<u64, u64> = HashMap::with_capacity(10);
    for word in input.split_whitespace() {
        let count = counts.entry(word.parse::<u64>().unwrap()).or_insert(0);
        *count += 1;
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

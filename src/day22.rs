
// https://adventofcode.com/2024/day/22

use rayon::prelude::*;
use std::collections::{HashSet,HashMap};

fn advance(i: u32) -> u32 {
    let o = (i <<  6) ^ i;
    let o = o & 0xFFFFFF;
    let o = (o >>  5) ^ o;
    let o = (o << 11) ^ o;
    o & 0xFFFFFF
}

fn advance_with_bytes(i: &mut u32, i_mod_10: &mut u32, pattern_bytes: &mut u32) {
    *i = advance(*i);
    let next_mod_10 = *i % 10;
    let next_byte = 10 + next_mod_10 - *i_mod_10;
    *pattern_bytes = *pattern_bytes << 8 | next_byte;
    *i_mod_10 = next_mod_10;
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> u64 {
    input.lines().par_bridge().map(|line| {
        let mut val = line.parse().unwrap();
        for _ in 0..2000 {
            val = advance(val);
        }
        val as u64
    }).sum()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u32 {
    let mut patterns: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let mut val = line.parse().unwrap();
        let mut val_mod_10 = val % 10;
        let mut pattern_bytes: u32 = 0;
        let mut my_patterns: HashSet<u32> = HashSet::new();
        for _ in 0..3 {
            advance_with_bytes(&mut val, &mut val_mod_10, &mut pattern_bytes);
        }
        for _ in 3..2000 {
            advance_with_bytes(&mut val, &mut val_mod_10, &mut pattern_bytes);
            if my_patterns.insert(pattern_bytes) {
                let p = patterns.entry(pattern_bytes).or_insert(0);
                *p += val_mod_10;
            }
        }
    }
    let mut max = 0;
    for (_, v) in patterns {
        if v > max {
            max = v;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!( 8685429, part1("1"));
        assert_eq!( 4700978, part1("10"));
        assert_eq!(15273692, part1("100"));
        assert_eq!( 8667524, part1("2024"));
        assert_eq!(37327623, part1("1\n10\n100\n2024"));

        assert_eq!(13753970725, part1(include_str!("../input/2024/day22.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(23, part2("1\n2\n3\n2024"));

        assert_eq!(1570, part2(include_str!("../input/2024/day22.txt")));
    }
}

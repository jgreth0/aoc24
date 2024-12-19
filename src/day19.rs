
// https://adventofcode.com/2024/day/19

use std::{collections::HashSet, hash::Hash};
use num_traits::int::PrimInt;
use rayon::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Towel<T> 
    where T: PrimInt + Hash + Eq
{
    code: T,
}

impl<T: PrimInt + Hash + Eq> Towel<T> {
    fn from_bytes(input: &[u8]) -> Self {
        let mut code: T = T::zero();
        // Hoping that the compiler understands this as an unaligned move and mask.
        for (i, b) in input.iter().enumerate() {
            code = code | T::from(*b).unwrap().unsigned_shl(8 * i as u32);
        }
        Towel { code }
    }
    fn from_bytes_set(input: &[u8], max_len: usize) -> Vec<Self> {
        let len = std::cmp::min(input.len(), max_len);
        let input = &input[0..len];
        let mut res = Vec::with_capacity(len);
        let mut code: T = T::zero();
        for (i, b) in input.iter().enumerate() {
            code = code | T::from(*b).unwrap().unsigned_shl(8 * i as u32);
            res.push(Towel { code });
        }
        res
    }
}

struct TowelSet<T>
    where T: PrimInt + Hash + Eq
{
    sets: Vec<HashSet<Towel<T>>>,
}

impl<T: PrimInt + Hash + Eq> TowelSet<T> {
    fn from_str(input: &str) -> Self {
        let mut sets = Vec::new();
        for item in input.split(", ") {
            while item.len() > sets.len() {
                sets.push(HashSet::new());
            }
            let set = &mut sets[item.len() - 1];
            set.insert(Towel::from_bytes(item.as_bytes()));
        }
        TowelSet { sets }
    }

    fn contains(&self, item: Towel<T>, len: usize) -> bool {
        self.sets[len-1].contains(&item)
    }

    fn count_builds(&self, line: &str) -> u64 {
        let line = line.as_bytes();
        let mut valid_starts = vec![0; line.len()+1];
        valid_starts[0] = 1;
        for i in 0..line.len() {
            if valid_starts[i] == 0 {
                continue;
            }
            let towels: Vec<Towel<T>> = Towel::from_bytes_set(&line[i..], self.sets.len());
            for (j, t) in towels.iter().enumerate() {
                if self.contains(*t, j+1) {
                    valid_starts[i+j+1] += valid_starts[i];
                }
            }
        }
        valid_starts[line.len()]
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u64 {
    let mut input = input.lines();
    let ts: TowelSet<u64> = TowelSet::from_str(input.next().unwrap());
    input.next();
    input.collect::<Vec<&str>>().par_iter().map(|line| {
        if ts.count_builds(line) > 0 { 1 } else { 0 }
    }).sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u64 {
    let mut input = input.lines();
    let ts: TowelSet<u64> = TowelSet::from_str(input.next().unwrap());
    input.next();
    input.collect::<Vec<&str>>().par_iter().map(|line| {
        ts.count_builds(line)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        r, wr, b, g, bwu, rb, gb, br\n\n\
        brwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";

    #[test]
    fn test_part1() {
        assert_eq!(6, part1(TEST_INPUT));

        assert_eq!(242, part1(include_str!("../input/2024/day19.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(16, part2(TEST_INPUT));

        assert_eq!(595975512785325, part2(include_str!("../input/2024/day19.txt")));
    }
}

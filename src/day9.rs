
// https://adventofcode.com/2024/day/9

use std::cmp::min;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let input: &[u8] = input.as_bytes();
    let mut in_pos: u32 = 0;
    let mut in_end: u32 = (input.len()-1) as u32;
    if in_end % 2 != 0 {
        in_end -= 1;
    }
    let mut out_pos: u32 = 0;
    let mut consumed_end: u32 = 0;
    let mut consumed_start: u32 = 0;
    let mut total: u64 = 0;
    loop {
        if in_end <= in_pos {
            if in_end < in_pos {
                break;
            }
            assert_eq!(in_end, in_pos);
            consumed_start = consumed_end;
        }
        let space: u32 = (input[in_pos as usize] - b'0') as u32 - consumed_start;
        assert!(space <= 9);
        if space == 0 {
            // If the whole space was consumed on the last iteration, in_pos
            // should have already advanced and consumed_start should have been
            // reset.
            assert_eq!(consumed_start, 0);
            in_pos += 1;
            continue;
        }
        if in_pos % 2 == 0 {
            // Count the file that is not moving.
            let id: u32 = in_pos / 2;
            let consume: u32 = space;
            assert!(consume <= 9);
            let consume_sum: u32 = (consume * (consume - 1)) / 2 + consume * out_pos;
            total += (id * consume_sum) as u64;
        
            out_pos += consume;

            in_pos += 1;
        } else {
            // Count the file that is moving.
            // in_end is always managed to be even.
            assert_eq!(in_end % 2, 0);
            let id: u32 = in_end / 2;
            let fill: u32 = (input[in_end as usize] - b'0') as u32 - consumed_end;
            assert!(fill <= 9);
            let consume: u32 = min(space, fill);
            assert!(consume <= 9);
            let consume_sum: u32 = (consume * (consume - 1)) / 2 + consume * out_pos;
            total += (id * consume_sum) as u64;
            out_pos += consume;

            consumed_start += consume;
            consumed_end += consume;

            if fill >= space {
                assert_eq!(consume, space);
                in_pos += 1;
                consumed_start = 0;
            }
            if space >= fill {
                assert_eq!(consume, fill);
                in_end -= 2;
                consumed_end = 0;
            }
        }
    }

    total
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let mut blocks: Vec<(u32, u32)> = Vec::with_capacity((input.len() + 1) / 2);
    let mut spaces: Vec<(u32, u32)> = Vec::with_capacity(input.len() / 2);
    let mut space_cache: [usize; 10] = [0; 10];
    let mut pos: u32 = 0;
    let mut total: u64 = 0;

    for (i, b) in input.as_bytes().iter().enumerate() {
        let len: u32 = (*b - b'0') as u32;
        assert!(len < 10);
        if i % 2 == 0 {
            blocks.push((pos, len));
        } else {
            spaces.push((pos, len));
        }
        pos += len;
    }

    for (id, (pos, len)) in blocks.iter().rev().enumerate() {
        let id = blocks.len() - id - 1;
        let mut new_pos = *pos;
        let cache: &mut usize = &mut space_cache[*len as usize];
        // TODO: Borrow from other caches?
        while *cache < id && spaces[*cache].1 < *len {
            *cache += 1;
        }
        if *cache < id {
            let space: &mut (u32, u32) = &mut spaces[*cache];
            new_pos = space.0;
            space.0 += *len;
            space.1 -= *len;
        }
        let consume_sum: u32 = (*len * (*len - 1)) / 2 + *len * new_pos;
        total += (id as u32 * consume_sum) as u64;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(1928, part1(TEST_INPUT));

        assert_eq!(6330095022244, part1(include_str!("../input/2024/day9.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2858, part2(TEST_INPUT));

        assert_eq!(6359491814941, part2(include_str!("../input/2024/day9.txt")));
    }
}

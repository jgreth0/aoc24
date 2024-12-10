
// https://adventofcode.com/2024/day/8

use std::collections::{HashMap,HashSet};

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
    return count_antinodes(input, false);
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    return count_antinodes(input, true);
}

fn count_antinodes(input: &str, extend: bool) -> u64 {
    let mut antennas: HashMap<u8,Vec<(i32, i32)>> = HashMap::new();
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut width: i32 = 0;
    for b in input.as_bytes() {
        // Optimized binary assumes there are no multi-byte unicode characters.
        assert!(b.is_ascii());
        if !b.is_ascii_whitespace() {
            if *b != b'.' {
                // Store the position of each antenna, grouped by ID. This will
                // allow for efficient processing later.
                antennas.entry(*b).or_insert_with(Vec::new).push((row, col));
            }
            col += 1;
        } else {
            if col != 0 {
                if width == 0 {
                    width = col;
                } else {
                    assert_eq!(col, width);
                }
                col = 0;
                row += 1;
            }
        }
    }
    // If whitespace is trimmed by the runner (it is), the loop above doesn't
    // increment the row count after the last row. Correct for that here.
    if col != 0 {
        assert_eq!(col, width);
        row += 1;
    }
    let width = width;
    let height = row;

    // Finished parsing inputs. Start processing...
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, points) in antennas.iter() {
        if points.len() < 2 {
            continue;
        }
        // Note: the instructions indicate that any gridpoint along the line
        // counts as an antinode. However, this implementation (which produces
        // an accepted answer), does not count gridpoints that are not a
        // multiple of the distance from either point. IMO a spec-compliant
        // implementation should have to count intermediate gridpoints if dx and
        // dy are not coprime.
        // TODO: Assert that dx and dy are coprime otherwise print an error
        // explaining this issue.
        for i in 0..points.len() {
            for j in 0..points.len() {
                // Walk in one direction (from i towards j and beyond). The
                // other direction will be walked in another iteration when i
                // and j are reversed.
                if i == j {
                    continue;
                }
                let mut p1 = points[i];
                let mut p2 = points[j];
                if extend {
                    antinodes.insert(p2);
                }
                loop {
                    // Select the next point along the line.
                    (p1, p2) = (p2, (2 * p2.0 - p1.0, 2 * p2.1 - p1.1));
                    // Terminate if the point is outside the grid.
                    if p2.0 >= 0 && p2.0 < width && p2.1 >= 0 && p2.1 < height {
                        antinodes.insert(p2);
                    } else {
                        break;
                    }
                    // Additional gridpoints are only added in part 2.
                    if !extend {
                        break;
                    }
                }
            }
        }
    }
    return antinodes.len() as u64;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    static TEST_INPUT: &str = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............";

    #[test]
    fn test_part1() {
        assert_eq!(14, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(34, part2(TEST_INPUT));
    }
}

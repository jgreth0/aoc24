
// https://adventofcode.com/2024/day/10

use std::collections::HashSet;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    let mut total: u64 = 0;

    // This structure stores the set of trail ends that are accessible from each
    // point.
    // TODO: don't copy bytes from input and instead wrap input with some
    // accessor methods.
    // TODO: The list of trail ends accessible from each point could be stored
    // as a bit vector instead of a HashSet. The set of trail ends accessible
    // from each point is limited to points that are within <10 manhattan
    // distance and on the correct checkerboard color. This means that only a
    // relatively small bit vector is required for each point (<128 bits
    // optimally or up less than 512 in a convenient layout).
    let mut grid: Vec<Vec<(u8, HashSet<(usize, usize)>)>> = input.trim_ascii_end().lines().map(|line| {
        line.as_bytes().iter().map(|b| {
            (*b, HashSet::new())
        }).collect()
    }).collect();

    for pass in (b'0'..b'0'+10).rev() {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x].0 != pass {
                    continue;
                }
                if pass == b'9' {
                    grid[y][x].1.insert((x, y));
                    continue;
                }
                
                if x > 0 {
                    if grid[y][x-1].0 == pass + 1 {
                        let c = grid[y][x-1].1.clone();
                        grid[y][x].1.extend(c);
                    }
                }
                if x + 1 < grid[y].len() {
                    if grid[y][x+1].0 == pass + 1 {
                        let c = grid[y][x+1].1.clone();
                        grid[y][x].1.extend(c);
                    }
                }
                if y > 0 {
                    if grid[y-1][x].0 == pass + 1 {
                        let c = grid[y-1][x].1.clone();
                        grid[y][x].1.extend(c);
                    }
                }
                if y + 1 < grid.len() {
                    if grid[y+1][x].0 == pass + 1 {
                        let c = grid[y+1][x].1.clone();
                        grid[y][x].1.extend(c);
                    }
                }
                
                if pass == b'0' {
                    total += grid[y][x].1.len() as u64;
                }
            }
        }
    }

    return total;
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;

    // TODO: This could be split into two separate vectors. Then the
    // accumulation later could be done with SIMD instructions.
    let mut grid: Vec<Vec<(u8, u64)>> = input.trim_ascii_end().lines().map(|line| {
        line.as_bytes().iter().map(|b| {
            (*b, 0)
        }).collect()
    }).collect();

    for pass in (b'0'..b'0'+10).rev() {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x].0 != pass {
                    continue;
                }
                if pass == b'9' {
                    grid[y][x].1 = 1;
                    continue;
                }
                
                if x > 0 {
                    if grid[y][x-1].0 == pass + 1 {
                        let c = grid[y][x-1].1;
                        grid[y][x].1 += c;
                    }
                }
                if x + 1 < grid[y].len() {
                    if grid[y][x+1].0 == pass + 1 {
                        let c = grid[y][x+1].1;
                        grid[y][x].1 += c;
                    }
                }
                if y > 0 {
                    if grid[y-1][x].0 == pass + 1 {
                        let c = grid[y-1][x].1;
                        grid[y][x].1 += c;
                    }
                }
                if y + 1 < grid.len() {
                    if grid[y+1][x].0 == pass + 1 {
                        let c = grid[y+1][x].1;
                        grid[y][x].1 += c;
                    }
                }
                
                if pass == b'0' {
                    total += grid[y][x].1;
                }
            }
        }
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_A: &str = "\
        0123\n\
        1234\n\
        8765\n\
        9876";

    static TEST_INPUT_B: &str = "\
        ...0...\n\
        ...1...\n\
        ...2...\n\
        6543456\n\
        7.....7\n\
        8.....8\n\
        9.....9";

    static TEST_INPUT_C: &str = "\
        ..90..9\n\
        ...1.98\n\
        ...2..7\n\
        6543456\n\
        765.987\n\
        876....\n\
        987....";

    static TEST_INPUT_D: &str = "\
        10..9..\n\
        2...8..\n\
        3...7..\n\
        4567654\n\
        ...8..3\n\
        ...9..2\n\
        .....01";

    static TEST_INPUT_E: &str = "\
        89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732";

    static TEST_INPUT_F: &str = "\
        .....0.\n\
        ..4321.\n\
        ..5..2.\n\
        ..6543.\n\
        ..7..4.\n\
        ..8765.\n\
        ..9....";

    static TEST_INPUT_G: &str = "\
        ..90..9\n\
        ...1.98\n\
        ...2..7\n\
        6543456\n\
        765.987\n\
        876....\n\
        987....";

    static TEST_INPUT_H: &str = "\
        012345\n\
        123456\n\
        234567\n\
        345678\n\
        4.6789\n\
        56789.";

    #[test]
    fn test_part1() {
        assert_eq!( 1, part1(TEST_INPUT_A));
        assert_eq!( 2, part1(TEST_INPUT_B));
        assert_eq!( 4, part1(TEST_INPUT_C));
        assert_eq!( 3, part1(TEST_INPUT_D));
        assert_eq!(36, part1(TEST_INPUT_E));

        assert_eq!(816, part1(include_str!("../input/2024/day10.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(  3, part2(TEST_INPUT_F));
        assert_eq!( 13, part2(TEST_INPUT_G));
        assert_eq!(227, part2(TEST_INPUT_H));
        assert_eq!( 81, part2(TEST_INPUT_E));

        assert_eq!(1960, part2(include_str!("../input/2024/day10.txt")));
    }
}

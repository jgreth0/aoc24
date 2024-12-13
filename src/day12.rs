
// https://adventofcode.com/2024/day/12

use std::collections::LinkedList;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u64 {
    // TODO: make a shared function with a part1/part2 select.
    let input: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
    // A work queue for finding all the entries in a blob.
    let mut queue: LinkedList<(i32, i32)> = LinkedList::new();
    let mut total = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let b = input[i][j];
            let mut perimeter = 0;
            let mut area = 0;
            // seed the work queue.
            queue.push_back((i as i32, j as i32));
            while let Some((k, l)) = queue.pop_front() {
                if k < 0 || k >= input.len() as i32 || l < 0 || l >= input[0].len() as i32 {
                    // if an out of bounds neighbor was added to the work queue,
                    // it means that whatever caused this to be added had a perimiter on this edge.
                    perimeter += 1;
                    continue;
                }
                let (ku, lu) = (k as usize, l as usize);
                if input[ku][lu] != b {
                    // Whatever caused this to be added had a perimiter on this edge.
                    perimeter += 1;
                    continue;
                }
                if seen[ku][lu] {
                    // If the first entry in the search was already seen,
                    // this continue will hit on the first iteration through 
                    // the work queue. Below, the total will be incremented by 0,
                    // so here is no double counting.
                    continue;
                }
                seen[ku][lu] = true;
                area += 1;
                queue.push_back((k-1, l));
                queue.push_back((k+1, l));
                queue.push_back((k, l-1));
                queue.push_back((k, l+1));
            }
            total += area * perimeter;
        }
    }
    return total;
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u64 {
    let input: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];
    // A work queue for finding all the entries in a blob.
    let mut queue: LinkedList<(usize, usize)> = LinkedList::new();
    let mut total = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let b = input[i][j];
            let mut perimeter = 0;
            let mut area = 0;
            // seed the work queue.
            queue.push_back((i, j));
            while let Some((k, l)) = queue.pop_front() {
                if seen[k][l] {
                    // If the first entry in the search was already seen,
                    // this continue will hit on the first iteration through 
                    // the work queue. Below, the total will be incremented by 0,
                    // so here is no double counting.
                    continue;
                }
                seen[k][l] = true;
                area += 1;
                
                // Count corners instead of edges.

                // Look up
                if k == 0 {
                    if l == 0 {
                        perimeter += 1; // Map corner
                    } else if input[k][l-1] != b {
                        perimeter += 1; // Convex corner
                    }
                } else {
                    if input[k-1][l] == b {
                        queue.push_back((k-1, l)); // Fill
                    } else if l == 0 {
                        perimeter += 1; // Convex corner
                    } else if input[k][l-1] != b {
                        perimeter += 1; // Convex corner
                    } else if input[k-1][l-1] == b {
                        perimeter += 1; // Concave corner
                    }
                }

                // Look down
                if k == input.len()-1 {
                    if l == 0 {
                        perimeter += 1; // Map corner
                    } else if input[k][l-1] != b {
                        perimeter += 1; // Convex corner
                    }
                } else {
                    if input[k+1][l] == b {
                        queue.push_back((k+1, l)); // Fill
                    } else if l == 0 {
                        perimeter += 1; // Convex corner
                    } else if input[k][l-1] != b {
                        perimeter += 1; // Convex corner
                    } else if input[k+1][l-1] == b {
                        perimeter += 1; // Concave corner
                    }
                }

                // Look left
                if l == 0 {
                    if k == 0 {
                        perimeter += 1; // Map corner
                    } else if input[k-1][l] != b {
                        perimeter += 1; // Convex corner
                    }
                } else {
                    if input[k][l-1] == b {
                        queue.push_back((k, l-1)); // Fill
                    } else if k == 0 {
                        perimeter += 1; // Convex corner
                    } else if input[k-1][l] != b {
                        perimeter += 1; // Convex corner
                    } else if input[k-1][l-1] == b {
                        perimeter += 1; // Concave corner
                    }
                }

                // Look right
                if l == input.len()-1 {
                    if k == 0 {
                        perimeter += 1; // Map corner
                    } else if input[k-1][l] != b {
                        perimeter += 1; // Convex corner
                    }
                } else {
                    if input[k][l+1] == b {
                        queue.push_back((k, l+1)); // Fill
                    } else if k == 0 {
                        perimeter += 1; // Convex corner
                    } else if input[k-1][l] != b {
                        perimeter += 1; // Convex corner
                    } else if input[k-1][l+1] == b {
                        perimeter += 1; // Concave corner
                    }
                }
            }
            total += area * perimeter;
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_A: &str = "\
        AAAA\n\
        BBCD\n\
        BBCC\n\
        EEEC";

    static TEST_INPUT_B: &str = "\
        OOOOO\n\
        OXOXO\n\
        OOOOO\n\
        OXOXO\n\
        OOOOO";

    static TEST_INPUT_C: &str = "\
        RRRRIICCFF\n\
        RRRRIICCCF\n\
        VVRRRCCFFF\n\
        VVRCCCJFFF\n\
        VVVVCJJCFE\n\
        VVIVCCJJEE\n\
        VVIIICJJEE\n\
        MIIIIIJJEE\n\
        MIIISIJEEE\n\
        MMMISSJEEE";

    static TEST_INPUT_D: &str = "\
        EEEEE\n\
        EXXXX\n\
        EEEEE\n\
        EXXXX\n\
        EEEEE";

    static TEST_INPUT_E: &str = "\
        AAAAAA\n\
        AAABBA\n\
        AAABBA\n\
        ABBAAA\n\
        ABBAAA\n\
        AAAAAA";

    #[test]
    fn test_part1() {
        assert_eq!( 140, part1(TEST_INPUT_A));
        assert_eq!( 772, part1(TEST_INPUT_B));
        assert_eq!(1930, part1(TEST_INPUT_C));

        assert_eq!(1471452, part1(include_str!("../input/2024/day12.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(  80, part2(TEST_INPUT_A));
        assert_eq!( 436, part2(TEST_INPUT_B));
        assert_eq!(1206, part2(TEST_INPUT_C));
        assert_eq!( 236, part2(TEST_INPUT_D));
        assert_eq!( 368, part2(TEST_INPUT_E));

        assert_eq!(863366, part2(include_str!("../input/2024/day12.txt")));
    }
}

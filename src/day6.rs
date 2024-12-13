
// https://adventofcode.com/2024/day/6

use std::collections::HashSet;
use rayon::prelude::*;

fn walk_length(obstacles: &HashSet<(i32, i32)>,
        extra_obstacle: &(i32, i32),
        bounds: &(i32, i32),
        guard_start_pos: &(i32, i32),
        guards: &mut Option<HashSet<(i32, i32)>>) -> u64 {
    let mut guard_dir: (i32, i32) = (0, -1);
    let mut guard_pos: (i32, i32) = *guard_start_pos;
    // This is a position and a direction tuple. It updates to the latest guard
    // pos/dir at turns, less frequently over time. Eventually, it will match
    // the current pos/dir if there is a loop.
    let mut loop_detect = (guard_pos, guard_dir);
    let mut loop_counter = 0;
    let mut loop_length = 1;
    if let Some(guards_set) = guards {
        guards_set.insert(guard_pos);
    }
    
    loop {
        let new_guard_pos: (i32, i32) = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);

        if new_guard_pos.0 < 0 || new_guard_pos.0 >= bounds.0 || new_guard_pos.1 < 0 || new_guard_pos.1 >= bounds.1 {
            break;
        }
        if obstacles.contains(&new_guard_pos) || new_guard_pos == *extra_obstacle {
            // Rotate 90 degrees.
            guard_dir = (-guard_dir.1, guard_dir.0);
            // Loop detection
            if loop_detect == (guard_pos, guard_dir) {
                return 1;
            }
            if loop_counter == loop_length {
                loop_length *= 2;
                loop_counter = 0;
                loop_detect = (guard_pos, guard_dir);
            }
            loop_counter += 1;
        } else {
            guard_pos = new_guard_pos;
            if let Some(guards_set) = guards {
                guards_set.insert(guard_pos);
            }
        }
    }

    0
}

// TODO: The input is parsed into a Set of obstacles and some metadata. A
// bit-vector would probably be substantially more efficient.
#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (HashSet<(i32, i32)>, (i32, i32), (i32, i32)) {
    let mut obstacles: HashSet<(i32, i32)> = HashSet::with_capacity(1000);

    let mut guard_pos: (i32, i32) = (-1, -1);
    let mut bounds: (i32, i32) = (0, 0);

    for line in input.lines() {
        let last_width = bounds.0;
        bounds.0 = 0;
        for c in line.chars() {
            match c {
                '#' => {
                    obstacles.insert(bounds);
                },
                '^' => {
                    guard_pos = bounds;
                },
                '.' => {},
                c if c.is_whitespace() => {
                    break;
                },
                _ => {
                    panic!("Invalid character: {}", c)
                }
            }
            bounds.0 += 1;
        }
        if bounds.0 == 0 {
            bounds.0 = last_width;
            break;
        }
        if bounds.1 != 0 {
            assert_eq!(last_width, bounds.0);
        }
        bounds.1 += 1;
    }
    (obstacles, guard_pos, bounds)
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
    let (obstacles, guard_pos, bounds) = parse_input(input);

    let mut guards: Option<HashSet<(i32, i32)>> = Some(HashSet::with_capacity(5000));
    walk_length(&obstacles, &(-1, -1), &bounds, &guard_pos, &mut guards);
    guards.unwrap().len() as u64
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
    let (obstacles, guard_pos, bounds) = parse_input(input);

    let mut guards: Option<HashSet<(i32, i32)>> = Some(HashSet::with_capacity(5000));
    walk_length(&obstacles, &(-1, -1), &bounds, &guard_pos, &mut guards);

    guards.unwrap().par_iter().map(|guard| -> u64 {
        if *guard == guard_pos {
            return 0;
        }
        // TODO: This replays the full walk for each possible guard pos. Instead
        // consider starting from the point in the original walk where the new
        // guard position was first considered.
        walk_length(&obstacles, guard, &bounds, &guard_pos, &mut None)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...";

    #[test]
    fn test_part1() {
        assert_eq!(41, part1(TEST_INPUT));

        assert_eq!(4559, part1(include_str!("../input/2024/day6.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(TEST_INPUT));

        assert_eq!(1604, part2(include_str!("../input/2024/day6.txt")));
    }
}

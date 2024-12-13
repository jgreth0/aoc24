
// https://adventofcode.com/2024/day/5

use std::collections::HashSet;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut pairs: HashSet<(u32, u32)> = HashSet::new();
    let mut total = 0;

    loop {
        if let Some(line) = lines.next() {
            let parts: Vec<&str> = line.split("|").collect();
            if parts.len() != 2 {
                break;
            }
            let a: u32 = parts[0].parse().unwrap();
            let b: u32 = parts[1].parse().unwrap();
            pairs.insert((a, b));
        } else {
            panic!("Reached eof before any update lines.");
        }
    }

    loop {
        if let Some(line) = lines.next() {
            if line.len() < 3 {
                break;
            }
            let update: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
            assert_eq!(update.len() % 2, 1);
            let mut order_ok = true;
            for a in 0..update.len()-1 {
                for b in a+1..update.len() {
                    if pairs.contains(&(update[b], update[a])) {
                        order_ok = false;
                    }
                }
            }
            if order_ok {
                total += update[update.len()/2];
                continue;
            }
        } else {
            break;
        }
    }
    return total;
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut pairs: HashSet<(u32, u32)> = HashSet::new();
    let mut total = 0;

    loop {
        if let Some(line) = lines.next() {
            let parts: Vec<&str> = line.split("|").collect();
            if parts.len() != 2 {
                break;
            }
            let a: u32 = parts[0].parse().unwrap();
            let b: u32 = parts[1].parse().unwrap();
            pairs.insert((a, b));
        } else {
            panic!("Reached eof before any update lines.");
        }
    }

    loop {
        if let Some(line) = lines.next() {
            if line.len() < 3 {
                break;
            }
            let mut update: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
            assert_eq!(update.len() % 2, 1);
            let mut order_ok = true;
            for a in 0..update.len()-1 {
                for b in a+1..update.len() {
                    if pairs.contains(&(update[b], update[a])) {
                        order_ok = false;
                    }
                }
            }
            if order_ok {
                continue;
            }
            for a in 0..update.len()-1 {
                order_ok = false;
                while !order_ok {
                    order_ok = true;
                    for b in a+1..update.len() {
                        if pairs.contains(&(update[b], update[a])) {
                            order_ok = false;
                            update.swap(a, b);
                            break;
                        }
                    }
                }
            }
            total += update[update.len()/2];
        } else {
            break;
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(143, part1(TEST_INPUT));

        assert_eq!(4774, part1(include_str!("../input/2024/day5.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, part2(TEST_INPUT));

        assert_eq!(6004, part2(include_str!("../input/2024/day5.txt")));
    }
}

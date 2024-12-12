
// https://adventofcode.com/2024/day/3
use std::iter::Peekable;

// TODO: This is a mess.
fn parse_do<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<bool> {
    if let Some(c) = iter.peek() {
        if *c == 'd' {
            iter.next();
        } else {
            return None;
        }
    } else {
        return None;
    }
    if let Some(c) = iter.peek() {
        if *c == 'o' {
            iter.next();
        } else {
            return None;
        }
    } else {
        return None;
    }
    if let Some(c) = iter.peek() {
        if *c == 'n' {
            iter.next();
        } else if *c == '(' {
            iter.next();
            if let Some(c) = iter.peek() {
                if *c == ')' {
                    return Some(true);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else {
        return None;
    }
    if let Some(c) = iter.peek() {
        if *c == '\'' {
            iter.next();
        } else {
            return None;
        }
    } else {
        return None;
    }
    if let Some(c) = iter.peek() {
        if *c == 't' {
            iter.next();
        } else {
            return None;
        }
    } else {
        return None;
    }
    if let Some(c) = iter.peek() {
        if *c == '(' {
            iter.next();
        } else {
            return None;
        }
    } else {
        return None;
    }
    if let Some(c) = iter.peek() {
        if *c == ')' {
            return Some(false);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

// TODO: This is a mess.
fn parse_mul<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> u32 {
    let mut a = 0;
    let mut b = 0;
    if let Some(c) = iter.peek() {
        if *c == 'm' {
            iter.next();
        } else {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if *c == 'u' {
            iter.next();
        } else {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if *c == 'l' {
            iter.next();
        } else {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if *c == '(' {
            iter.next();
        } else {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if let Some(val) = c.to_digit(10) {
            a = 10 * a + val;
            iter.next();
        }
        else {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if let Some(val) = c.to_digit(10) {
            a = 10 * a + val;
            iter.next();
        }
        else if *c != ',' {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if let Some(val) = c.to_digit(10) {
            a = 10 * a + val;
            iter.next();
        }
        else if *c != ',' {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if *c == ',' {
            iter.next();
        } else {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if let Some(val) = c.to_digit(10) {
            b = 10 * b + val;
            iter.next();
        }
        else {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if let Some(val) = c.to_digit(10) {
            b = 10 * b + val;
            iter.next();
        }
        else if *c != ')' {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if let Some(val) = c.to_digit(10) {
            b = 10 * b + val;
            iter.next();
        }
        else if *c != ')' {
            return 0;
        }
    } else {
        return 0;
    }
    if let Some(c) = iter.peek() {
        if *c == ')' {
            return a * b;
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut iter = line.chars().peekable();
        while let Some(_) = iter.peek() {
            let adder = parse_mul(&mut iter);
            total += adder;
            iter.next();
        }
    }
    return total;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut do_mode = true;

    for line in input.lines() {
        let mut iter = line.chars().peekable();
        while let Some(_) = iter.peek() {
            if let Some(mode) = parse_do(&mut iter) {
                do_mode = mode;
            }
            let adder = parse_mul(&mut iter);
            if do_mode {
                total += adder;
            }
            iter.next();
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_A: &str = "\
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static TEST_INPUT_B: &str = "\
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(TEST_INPUT_A));
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, part2(TEST_INPUT_B));
    }
}

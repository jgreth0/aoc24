
// https://adventofcode.com/2024/day/13

use scanf::sscanf;

fn get_total(input: &str, part2: bool) -> i64 {
    let mut x1: Vec<i64> = Vec::with_capacity(320);
    let mut y1: Vec<i64> = Vec::with_capacity(320);
    let mut x2: Vec<i64> = Vec::with_capacity(320);
    let mut y2: Vec<i64> = Vec::with_capacity(320);
    let mut x3: Vec<i64> = Vec::with_capacity(320);
    let mut y3: Vec<i64> = Vec::with_capacity(320);

    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        if let Some(b) = bytes.get(7) {
            if *b == b'A' {
                sscanf!(line, "Button A: X+{}, Y+{}", x, y).unwrap();
                x1.push(x);
                y1.push(y);
            } else if *b == b'B' {
                sscanf!(line, "Button B: X+{}, Y+{}", x, y).unwrap();
                x2.push(x);
                y2.push(y);
            } else if *b == b'X' {
                sscanf!(line, "Prize: X={}, Y={}", x, y).unwrap();
                if part2 {
                    x += 10000000000000;
                    y += 10000000000000;
                }
                x3.push(x);
                y3.push(y);
            }
        }
    }
    let count: usize = x1.len();
    let x1: &[i64] = &x1.as_slice()[0..count];
    let y1: &[i64] = &y1.as_slice()[0..count];
    let x2: &[i64] = &x2.as_slice()[0..count];
    let y2: &[i64] = &y2.as_slice()[0..count];
    let x3: &[i64] = &x3.as_slice()[0..count];
    let y3: &[i64] = &y3.as_slice()[0..count];
    let mut total: Vec<i64> = vec![0; count];
    let total: &mut [i64] = &mut total.as_mut_slice()[0..count];

    // Solve a system of equations for a and b:
    // a * x1 + b * x2 = x3
    // a * y1 + b * y2 = y3
    // a in 0..101
    // b in 0..101
    // If there are no solutions, return 0;
    // If there is 1 solution return 3 * a + b;
    // If there is more than one solution, return the smallest 3 * a + b;
    //
    // Consider the following transformations:
    // a * x1 * y2 + b * x2 * y2 = x3 * y2
    // a * x2 * y1 + b * x2 * y2 = x2 * y3
    // a * (x1 * y2 - x2 * y1) = (x3 * y2 - x2 * y3)
    // a = (x3 * y2 - x2 * y3) / (x1 * y2 - x2 * y1)
    // a = na / da
    // 
    // a * x1 * y1 + b * x2 * y1 = x3 * y1
    // a * x1 * y1 + b * x1 * y2 = x1 * y3
    // b * (x2 * y1 - x1 * y2) = (x3 * y1 - x1 * y3)
    // b = (x3 * y1 - x1 * y3) / (x2 * y1 - x1 * y2)
    // b = nb / db
    //
    // if na == da == nd == db == 0, there may be multiple solutions. Handle this case special.
    // else if da == 0 or db == 0, there are no solutions.
    // else there is one possible solution. Check if a and b are ints in the correct range.
    
    // TODO: Not sure what is preventing autovectorization here...
    for c in 0..count {
        let x1 = x1[c];
        let y1 = y1[c];
        let x2 = x2[c];
        let y2 = y2[c];
        let x3 = x3[c];
        let y3 = y3[c];
        let na = (x3 * y2) - (x2 * y3);
        let da = (x1 * y2) - (x2 * y1);
        let nb = (x3 * y1) - (x1 * y3);
        let db = 0 - da; // Note the expressions above are the same but reversed.
        assert!(da != 0);
        let a = na / da;
        let b = nb / db;
        total[c] = if 0 <= a && 0 <= b && a * da == na && b * db == nb {
            assert!(part2 || (a <= 100 && b <= 100));
            3 * a + b
        } else {
            0
        }
    }
        
    total.iter().sum()
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
    get_total(input, false)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    get_total(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        Button A: X+94, Y+34\n\
        Button B: X+22, Y+67\n\
        Prize: X=8400, Y=5400\n\
        \n\
        Button A: X+26, Y+66\n\
        Button B: X+67, Y+21\n\
        Prize: X=12748, Y=12176\n\
        \n\
        Button A: X+17, Y+86\n\
        Button B: X+84, Y+37\n\
        Prize: X=7870, Y=6450\n\
        \n\
        Button A: X+69, Y+23\n\
        Button B: X+27, Y+71\n\
        Prize: X=18641, Y=10279\n\
        \n";

    #[test]
    fn test_part1() {
        assert_eq!(480, part1(TEST_INPUT));

        assert_eq!(25751, part1(include_str!("../input/2024/day13.txt")));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(TEST_INPUT));

        assert_eq!(108528956728655, part2(include_str!("../input/2024/day13.txt")));
    }
}

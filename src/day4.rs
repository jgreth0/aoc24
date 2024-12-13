
// https://adventofcode.com/2024/day/4

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    let mut vec: Vec<Vec<char>> = Vec::new();
    let mut total = 0;

    for line in input.lines() {
        vec.push(line.chars().collect());
    }

    let xmas = ['X', 'M', 'A', 'S'];

    for x in 0..vec.len() {
        for y in 0..vec[x].len() {
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    let mut hit = true;
                    for i in 0..xmas.len() {
                        let tx = usize::try_from((x as i32) + dx * (i as i32));
                        let ty = usize::try_from((y as i32) + dy * (i as i32));
                        if let (Ok(txu), Ok(tyu)) = (tx, ty) {
                            if let Some(v2) = vec.get(txu) {
                                if let Some(c) = v2.get(tyu) {
                                    if *c != xmas[i] {
                                        hit = false;
                                    }
                                } else {
                                    hit = false;
                                }
                            } else {
                                hit = false;
                            }
                        } else {
                            hit = false;
                        }

                    }
                    if hit {
                        total += 1;
                    }
                }
            }
        }
    }
    return total;
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    let mut vec: Vec<Vec<char>> = Vec::new();
    let mut total = 0;

    for line in input.lines() {
        vec.push(line.chars().collect());
    }

    let mas = ['M', 'A', 'S'];

    for x in 0..vec.len() {
        for y in 0..vec[x].len() {
            for dx in [-1, 1] {
                for dy in [-1, 1] {
                    let mut hit = true;
                    for i in 0..mas.len() {
                        let tx = usize::try_from((x as i32) + dx * (i as i32));
                        let ty = usize::try_from((y as i32) + dy * (i as i32));
                        if let (Ok(txu), Ok(tyu)) = (tx, ty) {
                            if let Some(v2) = vec.get(txu) {
                                if let Some(c) = v2.get(tyu) {
                                    if *c != mas[i] {
                                        hit = false;
                                    }
                                } else {
                                    hit = false;
                                }
                            } else {
                                hit = false;
                            }
                        } else {
                            hit = false;
                        }
                    }
                    for i in 0..mas.len() {
                        let mut xi = i as i32;
                        let mut yi = i as i32;
                        if dx == dy {
                            xi = 2 - xi;
                        } else {
                            yi = 2 - yi;
                        }

                        let tx = usize::try_from((x as i32) + dx * xi);
                        let ty = usize::try_from((y as i32) + dy * yi);
                        if let (Ok(txu), Ok(tyu)) = (tx, ty) {
                            if let Some(v2) = vec.get(txu) {
                                if let Some(c) = v2.get(tyu) {
                                    if *c != mas[i] {
                                        hit = false;
                                    }
                                } else {
                                    hit = false;
                                }
                            } else {
                                hit = false;
                            }
                        } else {
                            hit = false;
                        }
                    }
                    if hit {
                        total += 1;
                    }
                }
            }
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(18, part1(TEST_INPUT));

        assert_eq!(2397, part1(include_str!("../input/2024/day4.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(TEST_INPUT));

        assert_eq!(1824, part2(include_str!("../input/2024/day4.txt")));
    }
}

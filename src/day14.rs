
// https://adventofcode.com/2024/day/14

use scanf::sscanf;

#[inline]
#[allow(clippy::too_many_arguments)]
unsafe fn process_line(input: &[u8], line_starts: &[usize], xb: i32, yb: i32, xbd2: i32, ybd2: i32, i: usize, checked: bool) -> u64 {
    let line_start = line_starts.get_unchecked(i);
    // Parse something like this: p=56,82 v=-79,-40
    let px_pos: usize = line_start + 2;
    let px_byte_0 = *input.get_unchecked(px_pos  ) as i32 - b'0' as i32;
    let px_byte_1 = *input.get_unchecked(px_pos+1) as i32 - b'0' as i32;
    let px_byte_2 = *input.get_unchecked(px_pos+2) as i32 - b'0' as i32;
    let px_if_1 = px_byte_0;
    let px_if_2 = px_byte_1 + 10 * px_if_1;
    let px_if_3 = px_byte_2 + 10 * px_if_2;
    let py_pos_if_1 = px_pos + 2;
    let py_pos_if_2 = px_pos + 3;
    let py_pos_if_3 = px_pos + 4;
    let (px, py_pos) =
        if px_byte_1 < 0 {
            (px_if_1, py_pos_if_1)
        } else if px_byte_2 < 0 {
            (px_if_2, py_pos_if_2)
        } else {
            (px_if_3, py_pos_if_3)
        };

    let py_byte_0 = *input.get_unchecked(py_pos  ) as i32 - b'0' as i32;
    let py_byte_1 = *input.get_unchecked(py_pos+1) as i32 - b'0' as i32;
    let py_byte_2 = *input.get_unchecked(py_pos+2) as i32 - b'0' as i32;
    let py_if_1 = py_byte_0;
    let py_if_2 = py_byte_1 + 10 * py_if_1;
    let py_if_3 = py_byte_2 + 10 * py_if_2;
    let vx_pos_if_1 = py_pos + 4;
    let vx_pos_if_2 = py_pos + 5;
    let vx_pos_if_3 = py_pos + 6;
    let (py, vx_pos) =
        if py_byte_1 < 0 {
            (py_if_1, vx_pos_if_1)
        } else if py_byte_2 < 0 {
            (py_if_2, vx_pos_if_2)
        } else {
            (py_if_3, vx_pos_if_3)
        };

    let vx_pos_3 = if checked && vx_pos+3 >= input.len() { py_pos-1 } else { vx_pos+3 };
    let vx_byte_0 = *input.get_unchecked(vx_pos  ) as i32 - b'0' as i32;
    let vx_byte_1 = *input.get_unchecked(vx_pos+1) as i32 - b'0' as i32;
    let vx_byte_2 = *input.get_unchecked(vx_pos+2) as i32 - b'0' as i32;
    let vx_byte_3 = *input.get_unchecked(vx_pos_3) as i32 - b'0' as i32;
    let vx_if_1  = vx_byte_0;
    let vx_if_2  = vx_byte_1 + 10 * vx_if_1;
    let vx_if_3  = vx_byte_2 + 10 * vx_if_2;
    let vx_if_n1 = xb - vx_byte_1;
    let vx_if_n2 = xb - vx_byte_2 + 10 * vx_if_n1;
    let vx_if_n3 = xb - vx_byte_3 + 10 * vx_if_n2;
    let vy_pos_if_1  = vx_pos + 2;
    let vy_pos_if_2  = vx_pos + 3;
    let vy_pos_if_3  = vx_pos + 4;
    let vy_pos_if_n1 = vx_pos + 3;
    let vy_pos_if_n2 = vx_pos + 4;
    let vy_pos_if_n3 = vx_pos + 5;
    let (vx, vy_pos) =
        if vx_byte_0 < 0 { // negative
            if vx_byte_2 < 0 {
                (vx_if_n1, vy_pos_if_n1)
            } else if vx_byte_3 < 0 {
                (vx_if_n2, vy_pos_if_n2)
            } else {
                (vx_if_n3, vy_pos_if_n3)
            }
        } else { // positive
            if vx_byte_1 < 0 {
                (vx_if_1, vy_pos_if_1)
            } else if vx_byte_2 < 0 {
                (vx_if_2, vy_pos_if_2)
            } else {
                (vx_if_3, vy_pos_if_3)
            }
        };

    let vy_pos_1 = if checked && vy_pos+1 >= input.len() { py_pos-1 } else { vy_pos+1 };
    let vy_pos_2 = if checked && vy_pos+2 >= input.len() { py_pos-1 } else { vy_pos+2 };
    let vy_pos_3 = if checked && vy_pos+3 >= input.len() { py_pos-1 } else { vy_pos+3 };
    let vy_byte_0 = *input.get_unchecked(vy_pos  ) as i32 - b'0' as i32;
    let vy_byte_1 = *input.get_unchecked(vy_pos_1) as i32 - b'0' as i32;
    let vy_byte_2 = *input.get_unchecked(vy_pos_2) as i32 - b'0' as i32;
    let vy_byte_3 = *input.get_unchecked(vy_pos_3) as i32 - b'0' as i32;
    let vy_if_1  = vy_byte_0;
    let vy_if_2  = vy_byte_1 + 10 * vy_if_1;
    let vy_if_3  = vy_byte_2 + 10 * vy_if_2;
    let vy_if_n1 = yb - vy_byte_1;
    let vy_if_n2 = yb - vy_byte_2 + 10 * vy_if_n1;
    let vy_if_n3 = yb - vy_byte_3 + 10 * vy_if_n2;
    let vy =
        if vy_byte_0 < 0 { // negative
            if vy_byte_2 < 0 {
                vy_if_n1
            } else if vy_byte_3 < 0 {
                vy_if_n2
            } else {
                vy_if_n3
            }
        } else { // positive
            if vy_byte_1 < 0 {
                vy_if_1
            } else if vy_byte_2 < 0 {
                vy_if_2
            } else {
                vy_if_3
            }
        };

    // Parsing is done. Now use (px, py, vx, vy) to calculate the final position
    // (npx, npy)

    let npx = px + (vx * 100);
    let npy = py + (vy * 100);
    core::hint::assert_unchecked(npx >= 0);
    core::hint::assert_unchecked(npy >= 0);
    core::hint::assert_unchecked(xb > 0);
    core::hint::assert_unchecked(yb > 0);
    let npx = npx % xb;
    let npy = npy % yb;

    // Set 1 in only one quadrant.
    let q1: u64 = if npx < xbd2 && npy < ybd2 { 1 } else { 0 };
    let q2: u64 = if npx > xbd2 && npy < ybd2 { 1 } else { 0 };
    let q3: u64 = if npx < xbd2 && npy > ybd2 { 1 } else { 0 };
    let q4: u64 = if npx > xbd2 && npy > ybd2 { 1 } else { 0 };

    // Store the quadrant selection in a u64 for efficiency. These can be summed
    // up without fear that values will roll over into another quadrant because
    // the total bots is ~500, well below 2^16 to cause a rollover.
    (q1 << 48) + (q2 << 32) + (q3 << 16) + q4
}

unsafe fn quad_prod_vec(input: &str, xb: i32, yb: i32) -> u64 {
    let input = input.as_bytes();
    let xbd2 = xb / 2;
    let ybd2 = yb / 2;
    // Lines are at least 12 bytes each.
    let mut line_starts: Vec<usize> = vec![0; input.len()/12];
    let line_starts: &mut [usize] = line_starts.as_mut_slice();
    let mut count = 0;
    let mut i = 0;
    while i < input.len() {
        if input[i] == b'p' {
            *line_starts.get_unchecked_mut(count) = i;
            count += 1;
            i += 12; // 12 is the shortest possible line length.
        } else {
            i += 1;
        }
    }
    let line_starts: &[usize] = line_starts;
    let count = count;

    // Candidate for autovectorization.
    let total_64: u64 = (0..count-1).map(|i| {
        let checked = false;
        process_line(input, line_starts, xb, yb, xbd2, ybd2, i, checked)
    }).sum();
    let total_64 = total_64 + process_line(input, line_starts, xb, yb, xbd2, ybd2, count-1, true);

    // Get separate values for the 4 quadrants and multiply them together.
    

    ((total_64 >> 48) & 0xffff) *
        ((total_64 >> 32) & 0xffff) *
        ((total_64 >> 16) & 0xffff) *
        ((total_64      ) & 0xffff)
}

fn quad_prod_vec_slow(input: &str, xb: i32, yb: i32) -> u64 {
    let xbd2 = xb / 2;
    let ybd2 = yb / 2;
    let total_64: u64 = input.lines().map(|line| {
        // Parse something like this: p=56,82 v=-79,-40
        let mut px: i32 = 0;
        let mut py: i32 = 0;
        let mut vx: i32 = 0;
        let mut vy: i32 = 0;
        sscanf!(line, "p={i32},{i32} v={i32},{i32}", px, py, vx, vy).unwrap();

        let npx = (px + (vx * 100)) % xb;
        let npx_pxb = npx + xb;
        let npx = if npx < 0 { npx_pxb } else { npx };
        let npy = (py + (vy * 100)) % yb;
        let npy_pyb = npy + yb;
        let npy = if npy < 0 { npy_pyb } else { npy };

        let q1: u64 = if npx < xbd2 && npy < ybd2 { 1 } else { 0 };
        let q2: u64 = if npx > xbd2 && npy < ybd2 { 1 } else { 0 };
        let q3: u64 = if npx < xbd2 && npy > ybd2 { 1 } else { 0 };
        let q4: u64 = if npx > xbd2 && npy > ybd2 { 1 } else { 0 };

        (q1 << 48) + (q2 << 32) + (q3 << 16) + q4
    }).sum();

    ((total_64 >> 48) & 0xffff) *
        ((total_64 >> 32) & 0xffff) *
        ((total_64 >> 16) & 0xffff) *
        ((total_64      ) & 0xffff)
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u64 {
    let res = unsafe { quad_prod_vec(input, 101, 103) };
    debug_assert_eq!(res, quad_prod_vec_slow(input, 101, 103));
    res
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> u64 {
    let do_print = false; // Use this for manual inspection.
    let xb = 101;
    let yb = 103;
    let mut bots: Vec<(i32, i32, i32, i32)> = Vec::with_capacity(512);
    for line in input.lines() {
        // Parse something like this: p=56,82 v=-79,-40
        let mut px: i32 = 0;
        let mut py: i32 = 0;
        let mut vx: i32 = 0;
        let mut vy: i32 = 0;
        sscanf!(line, "p={i32},{i32} v={i32},{i32}", px, py, vx, vy).unwrap();
        vx += xb;
        vy += yb;
        bots.push((px, py, vx, vy));
    };
    let mut count: u64 = 0;
    let mut printed = false;
    while !printed {
        printed = true;
        let mut map: Vec<Vec<char>> = vec![vec!['.'; 101]; 103];
        for bot in bots.iter_mut() {
            let m = map.get_mut(bot.1 as usize).unwrap().get_mut(bot.0 as usize).unwrap();
            // https://www.reddit.com/r/adventofcode/comments/1he88a8/
            if *m == '#' {
                printed = false;
            }
            *m = '#';
            bot.0 = (bot.0 + bot.2) % xb;
            bot.1 = (bot.1 + bot.3) % yb;
        }

        // https://www.reddit.com/r/adventofcode/comments/1he88a8/comment/m229vlf/
        if printed {
            printed = false;
            let mut consecutive = 0;
            for l in map.iter() {
                for c in l {
                    if *c == '.' {
                        consecutive = 0;
                    } else if consecutive > 8 {
                        printed = true;
                    } else {
                        consecutive += 1;
                    }
                }
                consecutive = 0;
            }

        }
        if printed {
            if do_print {
                println!("{}", count);
                for l in map {
                    for c in l {
                        print!("{}", c);
                    }
                    println!();
                }
                println!();
            }
        } else {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        p=0,4 v=3,-3\n\
        p=6,3 v=-1,-3\n\
        p=10,3 v=-1,2\n\
        p=2,0 v=2,-1\n\
        p=0,0 v=1,3\n\
        p=3,0 v=-2,-2\n\
        p=7,6 v=-1,-3\n\
        p=3,0 v=-1,-2\n\
        p=9,3 v=2,3\n\
        p=7,3 v=-1,2\n\
        p=2,4 v=2,-3\n\
        p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(12, unsafe { quad_prod_vec(TEST_INPUT, 11, 7) } );

        assert_eq!(226179492, part1(include_str!("../input/2024/day14.txt")));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(TEST_INPUT));

        assert_eq!(7502, part2(include_str!("../input/2024/day14.txt")));
    }
}

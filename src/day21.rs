
// https://adventofcode.com/2024/day/21

const DPAD_UP: usize = 0;
const DPAD_DOWN: usize = 1;
const DPAD_LEFT: usize = 2;
const DPAD_RIGHT: usize = 3;
const DPAD_A: usize = 4;

const DPAD_Y: [i64; 5] = [0, -1, -1, -1, 0];
const DPAD_X: [i64; 5] = [1, 1, 0, 2, 2];

const NUMPAD_Y: [i64; 11] = [0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 0];
const NUMPAD_X: [i64; 11] = [1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 2];

const fn xy_cost(x_tgt: i64, y_tgt: i64, x_src: i64, y_src: i64, dpad_costs: &[[u64; 5]; 5]) -> Option<u64> {
    let mut cost = 0;
    let mut x_src = x_src;
    let mut y_src = y_src;
    let mut current_pos = DPAD_A;
    while x_src < x_tgt {
        cost += dpad_costs[current_pos][DPAD_RIGHT];
        current_pos = DPAD_RIGHT;
        x_src += 1;
    }
    while x_tgt < x_src {
        cost += dpad_costs[current_pos][DPAD_LEFT];
        current_pos = DPAD_LEFT;
        x_src -= 1;
    }
    if x_src == 0 && y_src == 0 {
        return None;
    }
    while y_tgt < y_src {
        cost += dpad_costs[current_pos][DPAD_DOWN];
        current_pos = DPAD_DOWN;
        y_src -= 1;
    }
    while y_src < y_tgt {
        cost += dpad_costs[current_pos][DPAD_UP];
        current_pos = DPAD_UP;
        y_src += 1;
    }
    cost += dpad_costs[current_pos][DPAD_A];
    Some(cost)
}

const fn yx_cost(x_tgt: i64, y_tgt: i64, x_src: i64, y_src: i64, dpad_costs: &[[u64; 5]; 5]) -> Option<u64> {
    let mut cost = 0;
    let mut x_src = x_src;
    let mut y_src = y_src;
    let mut current_pos = DPAD_A;
    while y_tgt < y_src {
        cost += dpad_costs[current_pos][DPAD_DOWN];
        current_pos = DPAD_DOWN;
        y_src -= 1;
    }
    while y_src < y_tgt {
        cost += dpad_costs[current_pos][DPAD_UP];
        current_pos = DPAD_UP;
        y_src += 1;
    }
    if x_src == 0 && y_src == 0 {
        return None;
    }
    while x_src < x_tgt {
        cost += dpad_costs[current_pos][DPAD_RIGHT];
        current_pos = DPAD_RIGHT;
        x_src += 1;
    }
    while x_tgt < x_src {
        cost += dpad_costs[current_pos][DPAD_LEFT];
        current_pos = DPAD_LEFT;
        x_src -= 1;
    }
    cost += dpad_costs[current_pos][DPAD_A];
    Some(cost)
}

const DPAD_ROBOT_COST: [[[u64; 5]; 5]; 25] = {
    let mut rc: [[[u64; 5]; 5]; 25] = [[[0; 5]; 5]; 25];
    
    let mut start = 0;
    while start < 5 {
        let mut target = 0;
        while target < 5 {
            rc[0][start][target] += 1 + DPAD_X[target].abs_diff(DPAD_X[start]) + DPAD_Y[target].abs_diff(DPAD_Y[start]);
            target += 1;
        }
        start += 1;
    }
    let mut layer = 1;
    while layer < 25 {
        let mut start = 0;
        while start < 5 {
            let mut target = 0;
            while target < 5 {
                let x_tgt = DPAD_X[target];
                let y_tgt = DPAD_Y[target];
                let x_src = DPAD_X[start];
                let y_src = DPAD_Y[start];
                let xy_c = xy_cost(x_tgt, y_tgt, x_src, y_src, &rc[layer-1]);
                let yx_c = yx_cost(x_tgt, y_tgt, x_src, y_src, &rc[layer-1]);
                rc[layer][start][target] = match (xy_c, yx_c) {
                    (Some(xy), Some(yx)) => { if xy < yx { xy } else { yx } },
                    (None, Some(yx)) => yx,
                    (Some(xy), None) => xy,
                    (None, None) => panic!("Neither path is valid"),
                };
                target += 1;
            }
            start += 1;
        }
        layer += 1;
    }
    rc
};

const NUMPAD_MOVE_COST1: [[u64; 11]; 11] = {
    let mut rc: [[u64; 11]; 11] = [[0; 11]; 11];
    
    let mut start = 0;
    while start < 11 {
        let mut target = 0;
        while target < 11 {
            let x_tgt = NUMPAD_X[target];
            let y_tgt = NUMPAD_Y[target];
            let x_src = NUMPAD_X[start];
            let y_src = NUMPAD_Y[start];
            let xy_c = xy_cost(x_tgt, y_tgt, x_src, y_src, &DPAD_ROBOT_COST[1]);
            let yx_c = yx_cost(x_tgt, y_tgt, x_src, y_src, &DPAD_ROBOT_COST[1]);
            rc[start][target] = match (xy_c, yx_c) {
                (Some(xy), Some(yx)) => { if xy < yx { xy } else { yx } },
                (None, Some(yx)) => yx,
                (Some(xy), None) => xy,
                (None, None) => panic!("Neither path is valid"),
            };
            target += 1;
        }
        start += 1;
    }
    rc
};

const NUMPAD_MOVE_COST2: [[u64; 11]; 11] = {
    let mut rc: [[u64; 11]; 11] = [[0; 11]; 11];
    
    let mut start = 0;
    while start < 11 {
        let mut target = 0;
        while target < 11 {
            let x_tgt = NUMPAD_X[target];
            let y_tgt = NUMPAD_Y[target];
            let x_src = NUMPAD_X[start];
            let y_src = NUMPAD_Y[start];
            let xy_c = xy_cost(x_tgt, y_tgt, x_src, y_src, &DPAD_ROBOT_COST[24]);
            let yx_c = yx_cost(x_tgt, y_tgt, x_src, y_src, &DPAD_ROBOT_COST[24]);
            rc[start][target] = match (xy_c, yx_c) {
                (Some(xy), Some(yx)) => { if xy < yx { xy } else { yx } },
                (None, Some(yx)) => yx,
                (Some(xy), None) => xy,
                (None, None) => panic!("Neither path is valid"),
            };
            target += 1;
        }
        start += 1;
    }
    rc
};

const PART1_LUT: [u64; 4096] = {
    let mut lut: [u64; 4096] = [0; 4096];
    
    let mut i = 0;
    while i < 1000 {
        let mut nums: [usize; 3] = [0; 3];
        nums[0] = (i / 100) % 10;
        nums[1] = (i /  10) % 10;
        nums[2] = (i      ) % 10;
        // TODO
        let line: [u8; 4] = [nums[0] as u8 + b'0', nums[1] as u8 + b'0', nums[2] as u8 + b'0', b'A'];
        let line = u32::from_ne_bytes(line);
        let line = (line ^ (line >> 12)) & 0xFFF;
        let line = line as usize;
        lut[line] += NUMPAD_MOVE_COST1[     10][nums[0]];
        lut[line] += NUMPAD_MOVE_COST1[nums[0]][nums[1]];
        lut[line] += NUMPAD_MOVE_COST1[nums[1]][nums[2]];
        lut[line] += NUMPAD_MOVE_COST1[nums[2]][     10];
        lut[line] *= i as u64;
        i += 1;
    }

    lut
};

const PART2_LUT: [u64; 4096] = {
    let mut lut: [u64; 4096] = [0; 4096];
    
    let mut i = 0;
    while i < 1000 {
        let mut nums: [usize; 3] = [0; 3];
        nums[0] = (i / 100) % 10;
        nums[1] = (i /  10) % 10;
        nums[2] = (i      ) % 10;
        // TODO
        let line: [u8; 4] = [nums[0] as u8 + b'0', nums[1] as u8 + b'0', nums[2] as u8 + b'0', b'A'];
        let line = u32::from_ne_bytes(line);
        let line = (line ^ (line >> 12)) & 0xFFF;
        let line = line as usize;
        lut[line] += NUMPAD_MOVE_COST2[     10][nums[0]];
        lut[line] += NUMPAD_MOVE_COST2[nums[0]][nums[1]];
        lut[line] += NUMPAD_MOVE_COST2[nums[1]][nums[2]];
        lut[line] += NUMPAD_MOVE_COST2[nums[2]][     10];
        lut[line] *= i as u64;
        i += 1;
    }

    lut
};


#[aoc(day21, part1)]
pub fn part1(input: &str) -> u64 {
    input.lines().map(|line| {
        let line = line.as_bytes();
        let line: [u8; 4] = line[0..4].try_into().unwrap();
        let line = u32::from_ne_bytes(line);
        let line = (line ^ (line >> 12)) & 0xFFF;
        PART1_LUT[line as usize]
    }).sum()
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> u64 {
    input.lines().map(|line| {
        let line = line.as_bytes();
        let line: [u8; 4] = line[0..4].try_into().unwrap();
        let line = u32::from_ne_bytes(line);
        let line = (line ^ (line >> 12)) & 0xFFF;
        PART2_LUT[line as usize]
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "029A\n980A\n179A\n456A\n379A";

    #[test]
    fn test_part1() {
        // 999 distinct entries should be populated.
        assert_eq!(999, PART1_LUT.iter().map(|v| {if *v > 0 {1} else {0}}).sum());

        // Subsets of the example
        assert_eq!(68 *  29, part1("029A"));
        assert_eq!(60 * 980, part1("980A"));
        assert_eq!(68 * 179, part1("179A"));
        assert_eq!(64 * 456, part1("456A"));
        assert_eq!(64 * 379, part1("379A"));
        assert_eq!(126384, part1(TEST_INPUT));

        assert_eq!(137870, part1(include_str!("../input/2024/day21.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(170279148659464, part2(include_str!("../input/2024/day21.txt")));
    }
}

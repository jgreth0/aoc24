
// https://adventofcode.com/2024/day/7

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    // TODO: Move most of this into a shared function.
    let mut total: u64 = 0;
    for line in input.lines() {
        if !line.contains(":") {
            break;
        }
        let mut macro_parts = line.split(":");
        let res: u64 = macro_parts.next().unwrap().parse::<u64>().unwrap();
        let micro_parts = macro_parts.next().unwrap().split(" ");
        let mut vals: Vec<u64> = Vec::new();
        for p in micro_parts {
            if let Ok(val) = p.parse::<u64>() {
                vals.push(val);
            }
        }
        if has_solution2(res, &vals) {
            total += res;
        }
    }
    return total;
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let mut total: u64 = 0;
    // TODO: Rewrite as map+reduce
    for line in input.lines() {
        if !line.contains(":") {
            break;
        }
        // TODO: use match once for all parts.
        // TODO: Use collect() or similar to create the Vec.
        let mut macro_parts = line.split(":");
        let res: u64 = macro_parts.next().unwrap().parse::<u64>().unwrap();
        let micro_parts = macro_parts.next().unwrap().split(" ");
        let mut vals: Vec<u64> = Vec::new();
        for p in micro_parts {
            if let Ok(val) = p.parse::<u64>() {
                vals.push(val);
            }
        }
        if has_solution3(res, &vals) {
            total += res;
        }
    }
    return total;
}

// TODO: Merge into has_solution3
// TODO: Move res into vals[0]
// TODO: Return res or 0 instead of bool
fn has_solution2(res: u64, vals: &[u64]) -> bool {
    assert!(vals.len() > 0);
    assert!(res > 0);
    assert!(vals[vals.len()-1] > 0);

    if vals.len() == 1 {
        return res == vals[0];
    }
    if res % vals[vals.len()-1] == 0 {
        if has_solution2(res / vals[vals.len()-1], &vals[0..vals.len()-1]) {
            return true;
        }
    }
    if res > vals[vals.len()-1] {
        if has_solution2(res - vals[vals.len()-1], &vals[0..vals.len()-1]) {
            return true;
        }
    }
    return false;
}

fn has_solution3(res: u64, vals: &[u64]) -> bool {
    assert!(vals.len() > 0);
    assert!(res > 0);
    assert!(vals[vals.len()-1] > 0);

    if vals.len() == 1 {
        return res == vals[0];
    }
    if res % vals[vals.len()-1] == 0 {
        if has_solution3(res / vals[vals.len()-1], &vals[0..vals.len()-1]) {
            return true;
        }
    }
    if res > vals[vals.len()-1] {
        if has_solution3(res - vals[vals.len()-1], &vals[0..vals.len()-1]) {
            return true;
        }
        // TODO: This can be done without converting to strings
        let vs = vals[vals.len()-1].to_string();
        let rs = res.to_string();
        if (&rs).ends_with(&vs) {
            let new_rs = rs[..rs.len()-vs.len()].parse::<u64>().unwrap();
            if has_solution3(new_rs, &vals[0..vals.len()-1]) {
                return true;
            }
        }
    }
    return false;
}

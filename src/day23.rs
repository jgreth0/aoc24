
// https://adventofcode.com/2024/day/23

use std::collections::{HashMap,HashSet,LinkedList,BinaryHeap};

#[aoc(day23, part1)]
pub fn part1(input: &str) -> u64 {
    let mut tuples: HashMap<u16,HashSet<u16>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split("-");
        let a: [u8; 2] = parts.next().unwrap().as_bytes().try_into().unwrap();
        let a = u16::from_ne_bytes(a);
        let b: [u8; 2] = parts.next().unwrap().as_bytes().try_into().unwrap();
        let b = u16::from_ne_bytes(b);

        if a & 0xFF == b't' as u16 || a < b {
            tuples.entry(a).or_default().insert(b);
        }
        if b & 0xFF == b't' as u16 || b < a {
            tuples.entry(b).or_default().insert(a);
        }
    }
    let tuples = tuples;
    let mut total: u64 = 0;
    for (k1, v) in tuples.iter() {
        if k1 & 0xFF != b't' as u16 {
            continue;
        }
        for k2 in v.iter() {
            if k2 & 0xFF == b't' as u16 && k1 >= k2 {
                // Avoid double counting.
                continue;
            }
            for k3 in v.iter() {
                if k3 & 0xFF == b't' as u16 && k1 >= k3 {
                    // Avoid double counting.
                    continue;
                }
                if k2 >= k3 {
                    // avoid double counting.
                    continue;
                }
                if let Some(s) = tuples.get(k2){
                    if s.contains(k3) {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let mut tuples: HashMap<u16, HashSet<u16>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split("-");
        let a: [u8; 2] = parts.next().unwrap().as_bytes().try_into().unwrap();
        let a = u16::from_ne_bytes(a);
        let b: [u8; 2] = parts.next().unwrap().as_bytes().try_into().unwrap();
        let b = u16::from_ne_bytes(b);

        if a < b {
            tuples.entry(a).or_default().insert(b);
        }
        if b < a {
            tuples.entry(b).or_default().insert(a);
        }
    }

    let mut n_tuples: HashMap<Vec<u16>, LinkedList<u16>> = tuples
        .iter()
        .map(|(k, s)| (vec![*k], LinkedList::from_iter(s.iter().copied())))
        .collect();
    let mut n = 2;

    let mut next_n_tuples: HashMap<Vec<u16>, LinkedList<u16>> = HashMap::new();

    loop {
        for (k_prefix, v) in n_tuples.iter() {
            for k2 in v.iter() {
                for k3 in v.iter() {
                    if k2 >= k3 {
                        continue;
                    }
                    if let Some(s) = tuples.get(k2){
                        if s.contains(k3) {
                            let mut new_vec = Vec::with_capacity(n);
                            for i in k_prefix {
                                new_vec.push(*i);
                            }
                            new_vec.push(*k2);
                            next_n_tuples.entry(new_vec).or_default().push_back(*k3);
                        }
                    }
                }
            }
        }
        if next_n_tuples.is_empty() {
            break;
        }
        n_tuples = next_n_tuples;
        next_n_tuples = HashMap::new();
        n += 1;
    }
    for (k, v) in n_tuples.iter() {
        if let Some(l) = v.iter().next() {
            let mut bh: BinaryHeap<String> = BinaryHeap::from(k.iter().map(|w| {
                let bytes = u16::to_ne_bytes(*w);
                std::str::from_utf8(&bytes).unwrap().to_string()
            }).collect::<Vec<String>>());
            let bytes = u16::to_ne_bytes(*l);
            bh.push(std::str::from_utf8(&bytes).unwrap().to_string());
            let mut res = bh.pop().unwrap();
            while !bh.is_empty() {
                res = bh.pop().unwrap() + "," + res.as_str();
            }
            return res;
        }
    }
    panic!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\n\
        tb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\n\
        wh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\n\
        wq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn";

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(TEST_INPUT));

        assert_eq!(1230, part1(include_str!("../input/2024/day23.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!("co,de,ka,ta", part2(TEST_INPUT));

        assert_eq!("az,cj,kp,lm,lt,nj,rf,rx,sn,ty,ui,wp,zo", part2(include_str!("../input/2024/day23.txt")));
    }
}

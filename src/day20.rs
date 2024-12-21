
// https://adventofcode.com/2024/day/20

use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct MinPath{
    cost: i32,
}

impl MinPath {
    fn new() -> MinPath {
        MinPath {
            cost: i32::MAX,
        }
    }

    fn cost(&self) -> Option<i32> {
        if self.cost == i32::MAX {
            None
        } else {
            Some(self.cost)
        }
    }

    fn set_cost(&mut self, cost: i32) {
        self.cost = cost;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    End(MinPath),
    Wall,
    Path(MinPath),
}

impl Cell {
    fn from(b: u8) -> Self {
        match b {
            b'.' => Cell::Path(MinPath::new()),
            b'#' => Cell::Wall,
            b'S' => Cell::Path(MinPath::new()),
            b'E' => Cell::End(MinPath::new()),
            _ => panic!("Invalid cell"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn advance(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0-1, pos.1  ),
            Direction::South => (pos.0+1, pos.1  ),
            Direction::East  => (pos.0  , pos.1+1),
            Direction::West  => (pos.0  , pos.1-1),
        }
    }
}

struct PathNode {
    pos: (usize, usize),
    cost: i32,
}

impl PathNode {
    fn advanced_up(&self) -> Self {
        PathNode { 
            pos: Direction::North.advance(self.pos),
            cost: self.cost+1,
        }
    }
    fn advanced_down(&self) -> Self {
        PathNode { 
            pos: Direction::South.advance(self.pos),
            cost: self.cost+1,
        }
    }
    fn advanced_left(&self) -> Self {
        PathNode { 
            pos: Direction::East.advance(self.pos),
            cost: self.cost+1,
        }
    }
    fn advanced_right(&self) -> Self {
        PathNode { 
            pos: Direction::West.advance(self.pos),
            cost: self.cost+1,
        }
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Swap order to prioritize low-cost paths first.
        match self.cost.cmp(&other.cost) {
            std::cmp::Ordering::Less    => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal   => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for PathNode {}

struct Grid {
    cells: Vec<Vec<Cell>>,
    start_pos: (usize, usize),
    path: Vec<(usize, usize, i32)>,
}

impl Grid {
    fn from(input: &str) -> Self {
        let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(141);
        let mut start_pos = (0, 0);
        for line in input.lines() {
            let mut row = Vec::with_capacity(141);
            for b in line.bytes() {
                if b == b'S' {
                    start_pos = (cells.len(), row.len());
                }
                row.push(Cell::from(b));
            }
            cells.push(row);
        }
        Grid {
            cells,
            start_pos,
            path: Vec::new(),
        }
    }

    // Breadth-First Search of the grid, starting from the start position and
    // terminating when the end position is found.
    fn bfs(&mut self) -> Option<i32> {
        let mut queue: BinaryHeap<PathNode> = BinaryHeap::new();
        queue.push(PathNode {
            pos: self.start_pos,
            cost: 0,
        });
        while let Some(node) = queue.pop() {
            let cell = self.cells.get_mut(node.pos.0).unwrap().get_mut(node.pos.1).unwrap();
            match cell {
                Cell::End(c) => {
                    self.path.push((node.pos.0, node.pos.1, node.cost));
                    c.cost = node.cost;
                    return Some(node.cost);
                },
                Cell::Wall => continue,
                Cell::Path(m) => {
                    if let Some(c) = m.cost() {
                        debug_assert!(c <= node.cost, "Cost mismatch: {} vs {}", c, node.cost);
                        continue;
                    }
                    m.set_cost(node.cost);
                    self.path.push((node.pos.0, node.pos.1, node.cost));
                    queue.push(node.advanced_up());
                    queue.push(node.advanced_down());
                    queue.push(node.advanced_left());
                    queue.push(node.advanced_right());
                },
            }
        }
        None
    }

    // Check each pair of points in the path and count the pairs that are within
    // the maximum distance and provide at least the required savings for
    // cheating.
    // TODO: Try searching by proximity. For path-length N, this algo is O(N^2).
    // Searching by proximity would be O(N*max_distance^2), which might be
    // better especially for part 1 where max_distance is small.
    fn find_cheats(&self, min_savings: i32, max_distance: usize) -> u32 {
        let mut count = 0;
        for i in 0..self.path.len() {
            for j in 0.. self.path.len() {
                if i == j {
                    break;
                }
                let (sx, sy, sc) = self.path[j];
                let (ex, ey, ec) = self.path[i];
                let dist = ey.abs_diff(sy) + ex.abs_diff(sx);
                if dist <= max_distance && sc + (dist as i32) + min_savings <= ec {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_cheat_count(input: &str, min_savings: i32, max_distance: usize) -> u32 {
        let mut grid = Grid::from(input);
        grid.bfs();
        grid.find_cheats(min_savings, max_distance)
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> u32 {
    Grid::get_cheat_count(input, 100, 2)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> u32 {
    Grid::get_cheat_count(input, 100, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
        ###############\n\
        #...#...#.....#\n\
        #.#.#.#.#.###.#\n\
        #S#...#.#.#...#\n\
        #######.#.#.###\n\
        #######.#.#...#\n\
        #######.#.###.#\n\
        ###..E#...#...#\n\
        ###.#######.###\n\
        #...###...#...#\n\
        #.#####.#.###.#\n\
        #.#...#.#.#...#\n\
        #.#.#.#.#.#.###\n\
        #...#...#...###\n\
        ###############";

    #[test]
    fn test_part1() {
        assert_eq!(44, Grid::get_cheat_count(TEST_INPUT,  2, 2));
        assert_eq!(30, Grid::get_cheat_count(TEST_INPUT,  4, 2));
        assert_eq!(16, Grid::get_cheat_count(TEST_INPUT,  6, 2));
        assert_eq!(14, Grid::get_cheat_count(TEST_INPUT,  8, 2));
        assert_eq!(10, Grid::get_cheat_count(TEST_INPUT, 10, 2));
        assert_eq!( 8, Grid::get_cheat_count(TEST_INPUT, 12, 2));
        assert_eq!( 5, Grid::get_cheat_count(TEST_INPUT, 20, 2));
        assert_eq!( 4, Grid::get_cheat_count(TEST_INPUT, 36, 2));
        assert_eq!( 3, Grid::get_cheat_count(TEST_INPUT, 38, 2));
        assert_eq!( 2, Grid::get_cheat_count(TEST_INPUT, 40, 2));
        assert_eq!( 1, Grid::get_cheat_count(TEST_INPUT, 64, 2));

        assert_eq!(1321, part1(include_str!("../input/2024/day20.txt")));
    }

    #[test]
    fn test_part2() {

        assert_eq!(285, Grid::get_cheat_count(TEST_INPUT, 50, 20));
        assert_eq!(253, Grid::get_cheat_count(TEST_INPUT, 52, 20));
        assert_eq!(222, Grid::get_cheat_count(TEST_INPUT, 54, 20));
        assert_eq!(193, Grid::get_cheat_count(TEST_INPUT, 56, 20));
        assert_eq!(154, Grid::get_cheat_count(TEST_INPUT, 58, 20));
        assert_eq!(129, Grid::get_cheat_count(TEST_INPUT, 60, 20));
        assert_eq!(106, Grid::get_cheat_count(TEST_INPUT, 62, 20));
        assert_eq!( 86, Grid::get_cheat_count(TEST_INPUT, 64, 20));
        assert_eq!( 67, Grid::get_cheat_count(TEST_INPUT, 66, 20));
        assert_eq!( 55, Grid::get_cheat_count(TEST_INPUT, 68, 20));
        assert_eq!( 41, Grid::get_cheat_count(TEST_INPUT, 70, 20));
        assert_eq!( 29, Grid::get_cheat_count(TEST_INPUT, 72, 20));
        assert_eq!(  7, Grid::get_cheat_count(TEST_INPUT, 74, 20));
        assert_eq!(  3, Grid::get_cheat_count(TEST_INPUT, 76, 20));

        assert_eq!(971737, part2(include_str!("../input/2024/day20.txt")));
    }
}

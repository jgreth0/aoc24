
// https://adventofcode.com/2024/day/18

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
    End(i32),
    Wall,
    Path(MinPath, MinPath),
}

impl Cell {
    fn from(b: u8) -> Self {
        match b {
            b'.' => Cell::Path(MinPath::new(), MinPath::new()),
            b'#' => Cell::Wall,
            b'S' => Cell::Path(MinPath::new(), MinPath::new()),
            b'E' => Cell::End(i32::MAX),
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
}

impl Grid {
    fn from(input: &str, count: usize, dim: usize) -> Self {
        let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(dim+2);

        let start_pos = (1, 1);
        let end_pos = (dim, dim);
        cells.push(vec![Cell::from(b'#'); dim+2]);
        for _ in 0..dim {
            let mut row: Vec<Cell> = Vec::with_capacity(dim+2);
            row.push(Cell::from(b'#'));
            for _ in 0..dim {
                row.push(Cell::from(b'.'));
            }
            row.push(Cell::from(b'#'));
            cells.push(row);
        }
        cells.push(vec![Cell::from(b'#'); dim+2]);
        for (i, line) in input.lines().enumerate() {
            if i >= count {
                break;
            }
            let mut parsed = line.split(",").map(|s| s.parse::<usize>());
            let x = parsed.next().unwrap().expect("parse error");
            let y =  parsed.next().unwrap().expect("parse error");
            cells[y+1][x+1] = Cell::from(b'#');
        }
        cells[start_pos.0][start_pos.1] = Cell::from(b'S');
        cells[  end_pos.0][  end_pos.1] = Cell::from(b'E');
        Grid {
            cells,
            start_pos,
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
                    *c = node.cost;
                    return Some(node.cost);
                },
                Cell::Wall => continue,
                Cell::Path(m, _) => {
                    if let Some(c) = m.cost() {
                        debug_assert!(c <= node.cost, "Cost mismatch: {} vs {}", c, node.cost);
                        continue;
                    }
                    m.set_cost(node.cost);
                    queue.push(node.advanced_up());
                    queue.push(node.advanced_down());
                    queue.push(node.advanced_left());
                    queue.push(node.advanced_right());
                },
            }
        }
        None
    }

    // Binary search for the blocking wall.
    fn find_blocker(input: &str, dim: usize) -> String {
        let mut blocker_min = dim-2;
        let mut blocker_max = input.lines().count()-1;
        loop {
            if blocker_min == blocker_max {
                break;
            }
            let blocker = blocker_min + (blocker_max - blocker_min) / 2;
            if Grid::from(input, blocker, dim).bfs().is_none() {
                blocker_max = blocker;
            } else {
                blocker_min = blocker + 1
            }
        }
        input.lines().nth(blocker_min-1).unwrap().to_string()
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> i32 {
    Grid::from(input, 1024, 71).bfs().unwrap()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    Grid::find_blocker(input, 71)
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_A: &str = "\
        5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n\
        5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";

    #[test]
    fn test_part1() {
        assert_eq!(22, Grid::from(TEST_INPUT_A, 12, 7).bfs().unwrap());

        assert_eq!(374, part1(include_str!("../input/2024/day18.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!("6,1", Grid::find_blocker(TEST_INPUT_A, 7));

        assert_eq!("30,12", part2(include_str!("../input/2024/day18.txt")));
    }
}

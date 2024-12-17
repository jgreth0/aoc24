
// https://adventofcode.com/2024/day/16

use std::collections::{BinaryHeap,LinkedList};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct MinPath{
    north: i32,
    south: i32,
    east: i32,
    west: i32,
}

impl MinPath {
    fn new() -> MinPath {
        MinPath {
            north: i32::MAX,
            south: i32::MAX,
            east: i32::MAX,
            west: i32::MAX,
        }
    }

    fn direction_cost(&self, dir: Direction) -> Option<i32> {
        match dir {
            Direction::North => if self.north == i32::MAX { None } else { Some(self.north) },
            Direction::East  => if self.east  == i32::MAX { None } else { Some(self.east ) },
            Direction::South => if self.south == i32::MAX { None } else { Some(self.south) },
            Direction::West  => if self.west  == i32::MAX { None } else { Some(self.west ) },
        }
    }

    fn set_direction_cost(&mut self, dir: Direction, cost: i32) {
        match dir {
            Direction::North => self.north = cost,
            Direction::South => self.south = cost,
            Direction::East => self.east = cost,
            Direction::West => self.west = cost,
        }
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
    fn backup(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (pos.0+1, pos.1  ),
            Direction::South => (pos.0-1, pos.1  ),
            Direction::East  => (pos.0  , pos.1-1),
            Direction::West  => (pos.0  , pos.1+1),
        }
    }
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East  => Direction::North,
            Direction::West  => Direction::South,
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East  => Direction::South,
            Direction::West  => Direction::North,
        }
    }
}

struct PathNode {
    pos: (usize, usize),
    dir: Direction,
    cost: i32,
}

impl PathNode {
    fn advanced_forward(&self) -> Self {
        PathNode { 
            pos: self.dir.advance(self.pos), 
            dir: self.dir,
            cost: self.cost+1,
        }
    }
    fn advanced_left(&self) -> Self {
        PathNode { 
            pos: self.pos, 
            dir: self.dir.turn_left(),
            cost: self.cost+1000,
        }
    }
    fn advanced_right(&self) -> Self {
        PathNode { 
            pos: self.pos, 
            dir: self.dir.turn_right(),
            cost: self.cost+1000,
        }
    }
    fn backup_forward(&self) -> Self {
        PathNode { 
            pos: self.dir.backup(self.pos), 
            dir: self.dir,
            cost: self.cost-1,
        }
    }
    fn backup_left(&self) -> Self {
        PathNode { 
            pos: self.pos, 
            dir: self.dir.turn_right(),
            cost: self.cost-1000,
        }
    }
    fn backup_right(&self) -> Self {
        PathNode { 
            pos: self.pos, 
            dir: self.dir.turn_left(),
            cost: self.cost-1000,
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
    end_pos: (usize, usize),
}

impl Grid {
    fn from(input: &str) -> Self {
        let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(141);
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);
        for line in input.lines() {
            let mut row = Vec::with_capacity(141);
            for b in line.bytes() {
                if b == b'S' {
                    start_pos = (cells.len(), row.len());
                } else if b == b'E' {
                    end_pos = (cells.len(), row.len());
                }
                row.push(Cell::from(b));
            }
            cells.push(row);
        }
        Grid {
            cells,
            start_pos,
            end_pos,
        }
    }

    // Breadth-First Search of the grid, starting from the start position and
    // terminating when the end position is found.
    fn bfs(&mut self) -> i32 {
        let mut queue: BinaryHeap<PathNode> = BinaryHeap::new();
        queue.push(PathNode {
            pos: self.start_pos,
            dir: Direction::East,
            cost: 0,
        });
        while let Some(node) = queue.pop() {
            let cell = self.cells.get_mut(node.pos.0).unwrap().get_mut(node.pos.1).unwrap();
            match cell {
                Cell::End(c) => {
                    *c = node.cost;
                    return node.cost;
                },
                Cell::Wall => continue,
                Cell::Path(m, _) => {
                    if let Some(c) = m.direction_cost(node.dir) {
                        debug_assert!(c <= node.cost, "Cost mismatch: {} vs {}", c, node.cost);
                        continue;
                    }
                    m.set_direction_cost(node.dir, node.cost);
                    queue.push(node.advanced_forward());
                    queue.push(node.advanced_left());
                    queue.push(node.advanced_right());
                },
            }
        }
        panic!("No path found");
    }

    fn highlight_path(&mut self) -> i32 {
        let mut queue: LinkedList<PathNode> = LinkedList::new();
        let end = self.cells.get_mut(self.end_pos.0).unwrap().get_mut(self.end_pos.1).unwrap();
        let end_cost;
        if let Cell::End(c) = end {
            end_cost = *c;
        } else {
            panic!("End is not end");
        }
        let mut count = 1;
        for dir in [Direction::North, Direction::South, Direction::West, Direction::East] {
            queue.push_back(PathNode {
                pos: dir.backup(self.end_pos),
                dir,
                cost: end_cost-1,
            });
        }
        while let Some(node) = queue.pop_front() {
            let cell = self.cells.get_mut(node.pos.0).unwrap().get_mut(node.pos.1).unwrap();
            match cell {
                Cell::End(c) => {
                    debug_assert!(*c > node.cost, "Cost mismatch: {} vs {}", c, node.cost);
                    continue;
                },
                Cell::Wall => continue,
                Cell::Path(mp, cp) => {
                    if mp.direction_cost(node.dir) == cp.direction_cost(node.dir) {
                        // Already visited.
                        continue;
                    }
                    if let Some(c) = mp.direction_cost(node.dir) {
                        if c != node.cost {
                            // Not shortest path
                            continue;
                        }
                    }
                    if (cp.north, cp.south, cp.east, cp.west) == (i32::MAX, i32::MAX, i32::MAX, i32::MAX) {
                        count += 1;
                    }
                    cp.set_direction_cost(node.dir, node.cost);
                    queue.push_back(node.backup_forward());
                    queue.push_back(node.backup_left());
                    queue.push_back(node.backup_right());
                },
            }
        }
        count
    }
}

impl std::fmt::Display for Grid {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match *cell {
                    Cell::End(_) => { write!(f, "E")?; },
                    Cell::Wall => { write!(f, "#")?; },
                    Cell::Path(_, p) => { 
                        if self.start_pos == (y, x) {
                            write!(f, "S")?;
                        } else {
                            match (p.north, p.south, p.east, p.west) {
                                (i32::MAX, i32::MAX, i32::MAX, i32::MAX) =>
                                    { write!(f, ".")?; },
                                (i32::MAX, i32::MAX, i32::MAX, _) => 
                                    { write!(f, "<")?; },
                                (i32::MAX, i32::MAX, _, i32::MAX) =>
                                    { write!(f, ">")?; },
                                (i32::MAX, _, i32::MAX, i32::MAX) =>
                                    { write!(f, "v")?; },
                                (_, i32::MAX, i32::MAX, i32::MAX) =>
                                    { write!(f, "^")?; },
                                _ => 
                                    { write!(f, "+")?; },
                            }
                        }
                    },
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> i32 {
    let mut grid = Grid::from(input);
    // println!("{}", grid);
    #[allow(clippy::let_and_return)]
    let res = grid.bfs();
    // println!("{}", grid);
    res
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> i32 {
    let mut grid = Grid::from(input);
    // println!("{}", grid);
    let _ = grid.bfs();
    #[allow(clippy::let_and_return)]
    let res = grid.highlight_path();
    // println!("{}", grid);
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_A: &str = "\
        ###############\n\
        #.......#....E#\n\
        #.#.###.#.###.#\n\
        #.....#.#...#.#\n\
        #.###.#####.#.#\n\
        #.#.#.......#.#\n\
        #.#.#####.###.#\n\
        #...........#.#\n\
        ###.#.#####.#.#\n\
        #...#.....#.#.#\n\
        #.#.#.###.#.#.#\n\
        #.....#...#.#.#\n\
        #.###.#.#.#.#.#\n\
        #S..#.....#...#\n\
        ###############";

    static TEST_INPUT_B: &str = "\
        #################\n\
        #...#...#...#..E#\n\
        #.#.#.#.#.#.#.#.#\n\
        #.#.#.#...#...#.#\n\
        #.#.#.#.###.#.#.#\n\
        #...#.#.#.....#.#\n\
        #.#.#.#.#.#####.#\n\
        #.#...#.#.#.....#\n\
        #.#.#####.#.###.#\n\
        #.#.#.......#...#\n\
        #.#.###.#####.###\n\
        #.#.#...#.....#.#\n\
        #.#.#.#####.###.#\n\
        #.#.#.........#.#\n\
        #.#.#.#########.#\n\
        #S#.............#\n\
        #################";

    #[test]
    fn test_part1() {
        assert_eq!( 7036, part1(TEST_INPUT_A));
        assert_eq!(11048, part1(TEST_INPUT_B));

        assert_eq!(83432, part1(include_str!("../input/2024/day16.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45, part2(TEST_INPUT_A));
        assert_eq!(64, part2(TEST_INPUT_B));

        assert_eq!(467, part2(include_str!("../input/2024/day16.txt")));
    }
}

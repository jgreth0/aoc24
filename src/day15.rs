
// https://adventofcode.com/2024/day/15

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Robot,
    Empty,
    Wall,
    MovableBox,
    LeftMovableBox,
    RightMovableBox,
}

impl Cell {
    fn from(b: u8) -> Self {
        match b {
            b'.' => Cell::Empty,
            b'#' => Cell::Wall,
            b'O' => Cell::MovableBox,
            b'@' => Cell::Robot,
            _ => panic!("Invalid cell"),
        }
    }

    fn from_left(b: u8) -> Self {
        match b {
            b'.' => Cell::Empty,
            b'#' => Cell::Wall,
            b'O' => Cell::LeftMovableBox,
            b'@' => Cell::Robot,
            _ => panic!("Invalid cell"),
        }
    }

    fn from_right(b: u8) -> Self {
        match b {
            b'.' => Cell::Empty,
            b'#' => Cell::Wall,
            b'O' => Cell::RightMovableBox,
            b'@' => Cell::Empty,
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
    fn from(b: u8) -> Self {
        match b {
            b'^' => Direction::North,
            b'v' => Direction::South,
            b'>' => Direction::East,
            b'<' => Direction::West,
            _ => panic!("Invalid direction"),
        }
    }
    fn is_veritcal(&self) -> bool {
        match self {
            Direction::North | Direction::South => true,
            Direction::East | Direction::West => false,
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    robot_pos: (usize, usize),
    // double_wide: bool,
}

impl Grid {
    fn from(lines: &mut std::str::Lines<'_>, double_wide: bool) -> Self {
        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(50);
        let mut robot_pos = (0, 0);
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut row = Vec::with_capacity(if double_wide { 100 } else { 50 });
            for b in line.bytes() {
                if b == b'@' {
                    robot_pos = (grid.len(), row.len());
                }
                if double_wide {
                    row.push(Cell::from_left(b));
                    row.push(Cell::from_right(b));
                } else {
                    row.push(Cell::from(b));
                }
            }
            grid.push(row);
        }
        Grid {
            cells: grid,
            robot_pos,
            // double_wide: double_wide,
        }
    }
    
    // This is the core function that moves the robot and boxes around the grid.
    fn push_from(&mut self, pos: &mut (usize, usize), dir: &Direction, dry_run: bool) -> bool {
        match self.cells[pos.0][pos.1] {
            Cell::Empty => {
                return true;
            },
            Cell::Wall => {
                // A dry run is always tried first. If the dry run failed (a
                // wall was hit), an actual run would not be attempted.
                debug_assert!(dry_run);
                return false;
            },
            Cell::LeftMovableBox => {
                debug_assert_eq!(self.cells[pos.0][pos.1+1], Cell::RightMovableBox);
            },
            Cell::RightMovableBox => {
                debug_assert_eq!(self.cells[pos.0][pos.1-1], Cell::LeftMovableBox);
            },
            _ => {},
        }
        let new_pos = match *dir {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::West => (pos.0, pos.1 - 1),
        };
        let mut movable = true;
        
        let mut try_pos = new_pos;
        if !self.push_from(&mut try_pos, dir, dry_run) {
            movable = false;
        }
        
        // Move the second half of a box
        match (self.cells[pos.0][pos.1], self.cells[new_pos.0][new_pos.1], dir.is_veritcal()) {
            (Cell::LeftMovableBox, Cell::LeftMovableBox, true) | (Cell::RightMovableBox, Cell::RightMovableBox, true) => {
                // The displaced box is aligned with this one, so it was already moved. This is a no-op.
                // Consider if we did not skip this push during a dry run...
                // - The displaced box isn't actually moved because it is a dry run, so it will still be in its original place at this point.
                // - We will check if we can move the displaced box a second time.
                // - If there is a stack of N aligned boxes...
                //   - The first box will be checked once,
                //   - the second box will be checked twice,
                //   - the third box will be checked four times...
                //   - There will be 2^N-1 total calls! 
                //
                // This is imperfect though...
                // Consider the stack:
                //     v
                //     []
                //    [][]
                //     []
                // The bottom box will be checked twice even with this
                // optimization, once as a result of each box above it.
                // We could add a flag to track all of the checked boxes, but
                // then we would have to walk the tree a second time just to
                // clear all of the flags. This would have better worst-case
                // performance, but it would likely have worse average-case
                // performance, so it is not implemnted.
                //
                // TODO: Consider implementing this optimization to try if there
                // is a perf issue.
            }
            (Cell::LeftMovableBox, _, true) => {
                let mut new_right_pos = new_pos;
                new_right_pos.1 += 1;
                if !self.push_from(&mut new_right_pos, dir, dry_run) {
                    movable = false;
                }
            },
            (Cell::RightMovableBox, _, true) => {
                let mut new_left_pos = new_pos;
                new_left_pos.1 -= 1;
                if !self.push_from(&mut new_left_pos, dir, dry_run) {
                    movable = false;
                }
            },
            _ => {}, // A few panic cases
        }

        if dry_run {
            return movable;
        }
        if !movable {
            panic!("Not a dry run, but cannot move.")
        }

        // Move the box/robot
        match self.cells[pos.0][pos.1] {
            Cell::MovableBox | Cell::Robot => {
                debug_assert_eq!(self.cells[new_pos.0][new_pos.1], Cell::Empty);
                self.cells[new_pos.0][new_pos.1] = self.cells[pos.0][pos.1];
                self.cells[    pos.0][    pos.1] = Cell::Empty;
                *pos = new_pos;
            },
            Cell::LeftMovableBox => {
                debug_assert_eq!(self.cells[new_pos.0][new_pos.1], Cell::Empty);
                self.cells[new_pos.0][new_pos.1] = Cell::LeftMovableBox;
                self.cells[    pos.0][    pos.1] = Cell::Empty;
                if dir.is_veritcal() {
                    debug_assert_eq!(self.cells[new_pos.0][new_pos.1+1], Cell::Empty);
                    self.cells[new_pos.0][new_pos.1+1] = Cell::RightMovableBox;
                    self.cells[    pos.0][    pos.1+1] = Cell::Empty;
                }
                *pos = new_pos;
            },
            Cell::RightMovableBox => {
                debug_assert_eq!(self.cells[new_pos.0][new_pos.1], Cell::Empty);
                self.cells[new_pos.0][new_pos.1] = Cell::RightMovableBox;
                self.cells[    pos.0][    pos.1] = Cell::Empty;
                if dir.is_veritcal() {
                    debug_assert_eq!(self.cells[new_pos.0][new_pos.1-1], Cell::Empty);
                    self.cells[new_pos.0][new_pos.1-1] = Cell::LeftMovableBox;
                    self.cells[    pos.0][    pos.1-1] = Cell::Empty;
                }
                *pos = new_pos;
            },
            Cell::Empty | Cell::Wall => {
                panic!("Nothing to move, should have returned earlier");
            },
        }
        true
    }
    fn push_robot(&mut self, dir: Direction) {
        let mut robot_pos = self.robot_pos;
        if self.push_from(&mut robot_pos, &dir, true) {
            self.push_from(&mut robot_pos, &dir, false);
        }
        self.robot_pos = robot_pos;
    }
    fn coordinate_sum(&self) -> u64 {
        let mut sum = 0;
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::MovableBox | Cell::LeftMovableBox => { sum += i * 100 + j; },
                    _ => {},
                };
            }
        }
        sum as u64
    }
}

impl std::fmt::Display for Grid {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match *cell {
                    Cell::MovableBox => { write!(f, "O")?; },
                    Cell::LeftMovableBox => { write!(f, "[")?; },
                    Cell::RightMovableBox => { write!(f, "]")?; },
                    Cell::Empty => { write!(f, ".")?; },
                    Cell::Wall => { write!(f, "#")?; },
                    Cell::Robot => { write!(f, "@")?; },
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut grid = Grid::from(&mut lines, false);
    for line in lines {
        for b in line.bytes() {
            grid.push_robot(Direction::from(b));
        }
    }
    grid.coordinate_sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut grid = Grid::from(&mut lines, true);
    //print!("{}", grid);
    for line in lines {
        for b in line.bytes() {
            grid.push_robot(Direction::from(b));
        }
    }
    //print!("{}", grid);
    grid.coordinate_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_A: &str = "\
        ########\n\
        #..O.O.#\n\
        ##@.O..#\n\
        #...O..#\n\
        #.#.O..#\n\
        #...O..#\n\
        #......#\n\
        ########\n\
        \n\
        <^^>>>vv<v>>v<<";

    static TEST_INPUT_B: &str = "\
        ##########\n\
        #..O..O.O#\n\
        #......O.#\n\
        #.OO..O.O#\n\
        #..O@..O.#\n\
        #O#..O...#\n\
        #O..O..O.#\n\
        #.OO.O.OO#\n\
        #....O...#\n\
        ##########\n\
        \n\
        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        assert_eq!(2028, part1(TEST_INPUT_A));
        assert_eq!(10092, part1(TEST_INPUT_B));

        assert_eq!(1514333, part1(include_str!("../input/2024/day15.txt")));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9021, part2(TEST_INPUT_B));

        assert_eq!(1528453, part2(include_str!("../input/2024/day15.txt")));
    }
}

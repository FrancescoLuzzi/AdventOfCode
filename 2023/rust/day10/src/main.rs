use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::{Add, AddAssign};
use std::rc::Rc;

use derive_more::Display;

#[derive(Clone, Copy, Debug)]
struct Piece {
    point: Point,
    _type: PieceType,
    direction: Option<Direction>,
    distance: isize,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Add for Point {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Point { x: 0, y: 0 };
        out.x = self.x.checked_add(rhs.x)?;
        out.y = self.y.checked_add(rhs.y)?;
        Some(out)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.checked_add(rhs.x).unwrap();
        self.y = self.y.checked_add(rhs.y).unwrap();
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Option<Self>;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        let mut out = Point { x: 0, y: 0 };
        out.x = self.x.checked_add_signed(rhs.0)?;
        out.y = self.y.checked_add_signed(rhs.1)?;
        Some(out)
    }
}

impl AddAssign<(isize, isize)> for Point {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        self.x = self.x.checked_add_signed(rhs.0).unwrap();
        self.y = self.y.checked_add_signed(rhs.1).unwrap();
    }
}

impl Add<Direction> for Point {
    type Output = Option<Self>;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.next_offset()
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self += rhs.next_offset();
    }
}

#[derive(Clone, Copy, Display, Debug)]
enum Direction {
    #[display(fmt = "North")]
    North,
    #[display(fmt = "South")]
    South,
    #[display(fmt = "West")]
    West,
    #[display(fmt = "East")]
    East,
}

impl Direction {
    fn next_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

#[derive(Display, Debug, Clone, Copy)]
enum PieceType {
    /// |
    #[display(fmt = "|")]
    Vertical,
    /// |
    #[display(fmt = "-")]
    Orizzontal,
    /// L
    // #[display(fmt = "L")]
    #[display(fmt = "╰")]
    BendNE,
    /// F
    // #[display(fmt = "F")]
    #[display(fmt = "╭")]
    BendSE,
    /// 7
    // #[display(fmt = "7")]
    #[display(fmt = "╮")]
    BendSW,
    /// J
    // #[display(fmt = "J")]
    #[display(fmt = "╯")]
    BendNW,
    /// .
    #[display(fmt = ".")]
    Ground,
    /// .
    #[display(fmt = "I")]
    Inside,
    /// S
    #[display(fmt = "🐱")]
    Animal,
}

impl From<char> for PieceType {
    fn from(value: char) -> Self {
        match value {
            '|' => PieceType::Vertical,
            '-' => PieceType::Orizzontal,
            'L' => PieceType::BendNE,
            'F' => PieceType::BendSE,
            '7' => PieceType::BendSW,
            'J' => PieceType::BendNW,
            '.' => PieceType::Ground,
            'S' => PieceType::Animal,
            'I' => PieceType::Inside,
            _ => PieceType::Ground,
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum DirectionError {
    #[error("This is a ground piece")]
    Ground,
    #[error("This the animal piece")]
    Animal,
    #[error("Impossible move to {1} with piece {0}")]
    Impossible(PieceType, Direction),
}

impl PieceType {
    fn get_next_direction(&self, prev_dir: &Direction) -> Result<Direction, DirectionError> {
        match self {
            PieceType::Vertical => match prev_dir {
                Direction::North => Ok(Direction::North),
                Direction::South => Ok(Direction::South),
                dir => Err(DirectionError::Impossible(*self, *dir)),
            },
            PieceType::Orizzontal => match prev_dir {
                Direction::East => Ok(Direction::East),
                Direction::West => Ok(Direction::West),
                dir => Err(DirectionError::Impossible(*self, *dir)),
            },
            PieceType::BendNE => match prev_dir {
                Direction::South => Ok(Direction::East),
                Direction::West => Ok(Direction::North),
                dir => Err(DirectionError::Impossible(*self, *dir)),
            },
            PieceType::BendSE => match prev_dir {
                Direction::West => Ok(Direction::South),
                Direction::North => Ok(Direction::East),
                dir => Err(DirectionError::Impossible(*self, *dir)),
            },
            PieceType::BendSW => match prev_dir {
                Direction::East => Ok(Direction::South),
                Direction::North => Ok(Direction::West),
                dir => Err(DirectionError::Impossible(*self, *dir)),
            },
            PieceType::BendNW => match prev_dir {
                Direction::East => Ok(Direction::North),
                Direction::South => Ok(Direction::West),
                dir => Err(DirectionError::Impossible(*self, *dir)),
            },
            PieceType::Ground | PieceType::Inside => Err(DirectionError::Ground),
            PieceType::Animal => Err(DirectionError::Animal),
        }
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<Piece>>);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            for piece in line.iter() {
                f.write_fmt(format_args!("{}", piece._type))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Map {
    fn get_mut(&mut self, point: &Point) -> Option<&mut Piece> {
        self.0.get_mut(point.y)?.get_mut(point.x)
    }

    fn get_max_distance(&self) -> isize {
        self.0
            .iter()
            .flat_map(|x| x.iter().map(|y| y.distance))
            .max()
            .unwrap_or(-1)
    }

    fn reset_non_loop_pipes(&mut self) -> bool {
        if self.get_max_distance() < 1 {
            return false;
        }
        for line in self.0.iter_mut() {
            for piece in line.iter_mut() {
                if piece.distance == -1 || piece.direction.is_none() {
                    piece._type = PieceType::Ground;
                }
            }
        }
        true
    }

    // raycasting algorithm
    // walk orizzontally every row
    // when stepping on ground, that piece is:
    // - outside the loop, if the number of "vertical" pipes passed is even
    // - inside the loop, if the number of "vertical" pipes passed is odd
    // Vertical pipes are defined as:
    // - `|`
    // - `F--J`
    // - `L--7`
    // Orizzontal pipes are defined as:
    // - `-`
    // - `F--7`
    // - `L--J`
    fn get_num_pieces_inside_the_loop(&mut self) -> Option<usize> {
        if self.get_max_distance() < 1 {
            return None;
        }
        let mut inside_pieces: usize = 0;
        let mut prev_piece_type = PieceType::Ground;
        for line in self.0.iter_mut() {
            let mut pipes_passed: usize = 0;
            for piece in line.iter_mut() {
                match piece._type {
                    PieceType::Vertical => pipes_passed += 1,
                    PieceType::BendNE | PieceType::BendSE => prev_piece_type = piece._type,
                    PieceType::BendSW => {
                        if matches!(prev_piece_type, PieceType::BendNE) {
                            pipes_passed += 1;
                        }
                    }
                    PieceType::BendNW => {
                        if matches!(prev_piece_type, PieceType::BendSE) {
                            pipes_passed += 1;
                        }
                    }
                    PieceType::Orizzontal => {}
                    PieceType::Ground => {
                        if pipes_passed % 2 != 0 {
                            inside_pieces += 1;
                            piece._type = PieceType::Inside;
                        }
                    }
                    //impossible
                    PieceType::Animal | PieceType::Inside => {}
                }
            }
        }
        Some(inside_pieces)
    }
}

fn main() {
    let input_lines = aoc_utils::load_input_file("input.txt");
    let mut animal = Piece {
        point: Point { x: 0, y: 0 },
        _type: PieceType::Ground,
        direction: None,
        distance: 0,
    };
    let matrix = input_lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let mut tmp = Piece {
                        point: Point { x, y },
                        _type: c.into(),
                        direction: None,
                        distance: -1,
                    };
                    if let PieceType::Animal = tmp._type {
                        tmp.distance = 0;
                        animal = tmp;
                    }
                    tmp
                })
                .collect::<Vec<Piece>>()
        })
        .collect::<Vec<Vec<Piece>>>();

    let matrix: Rc<RefCell<Map>> = Rc::new(RefCell::new(Map(matrix)));

    match animal._type {
        PieceType::Animal => {}
        _ => panic!("animal not found"),
    }
    // to have a loop we need 2 valid starts
    let mut to_explore: VecDeque<Piece> = VecDeque::new();
    for dir in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        let next_point = animal.point + dir;
        if next_point.is_none() {
            continue;
        }
        let mut tmp_matrix = matrix.borrow_mut();
        let next_piece = tmp_matrix.get_mut(&next_point.unwrap());
        if next_piece.is_none() {
            continue;
        }
        let next_piece = next_piece.unwrap();
        if next_piece._type.get_next_direction(&dir).is_ok() {
            next_piece.direction = Some(dir);
            next_piece.distance = 1;
            to_explore.push_back(*next_piece);
        }
    }
    if to_explore.len() != 2 {
        panic!("did not found 2 starts");
    }
    {
        //figure out the real piece_type of the animal
        let start_directions: Vec<Direction> =
            to_explore.iter().flat_map(|x| x.direction).collect();
        match start_directions[..] {
            [Direction::North, Direction::South] => {
                animal._type = PieceType::Vertical;
                animal.direction = Some(Direction::North);
            }
            [Direction::East, Direction::West] => {
                animal._type = PieceType::Orizzontal;
                animal.direction = Some(Direction::East);
            }
            [Direction::North, Direction::East] => {
                animal._type = PieceType::BendNE;
                animal.direction = Some(Direction::North);
            }
            [Direction::North, Direction::West] => {
                animal._type = PieceType::BendNW;
                animal.direction = Some(Direction::North);
            }
            [Direction::South, Direction::West] => {
                animal._type = PieceType::BendSW;
                animal.direction = Some(Direction::South);
            }
            [Direction::East, Direction::South] => {
                animal._type = PieceType::BendSE;
                animal.direction = Some(Direction::East);
            }
            _ => panic!("What is that!?!?"),
        }
        // then replace it in the matrix
        let mut tmp_matrix = matrix.borrow_mut();
        let tmp_animal = tmp_matrix.get_mut(&animal.point).unwrap();
        tmp_animal._type = animal._type;
        tmp_animal.direction = animal.direction;
        tmp_animal.distance = 0;
    }

    // BFS looking for the furthest point
    while !to_explore.is_empty() {
        let curr = to_explore.pop_front().unwrap();
        let prev_distance = curr.distance;
        let next_dir = curr
            ._type
            .get_next_direction(&curr.direction.unwrap())
            .unwrap();
        let mut tmp_matrix = matrix.borrow_mut();
        let curr = tmp_matrix
            .get_mut(&(curr.point + next_dir).unwrap())
            .unwrap();
        if curr.direction.is_none() && curr.distance < 0 {
            curr.direction = Some(next_dir);
            curr.distance = prev_distance + 1;
            to_explore.push_back(*curr);
        }
    }
    matrix.borrow_mut().reset_non_loop_pipes();

    let part2 = matrix
        .borrow_mut()
        .get_num_pieces_inside_the_loop()
        .unwrap_or(0);
    println!("{}", matrix.borrow());
    println!("part1= {}", matrix.borrow().get_max_distance());
    println!("part2= {}", part2);
}

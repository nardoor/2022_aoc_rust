#![feature(iter_array_chunks)]
// 🎄 Part 1 🎄
// 75254 (elapsed: 4.09ms)
// 🎄 Part 2 🎄
// 108311 (elapsed: 439.52µs)
use core::panic;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Write},
    ops::{Add, AddAssign, Deref, DerefMut},
    rc::{Rc, Weak},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, char, digit1},
    combinator::map,
    multi::{many0, many1},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Void,
    Wall,
    Floor,
}

trait TileTrait {
    fn is_void(&self) -> bool;
    fn is_wall(&self) -> bool;
}

impl TileTrait for Tile {
    fn is_void(&self) -> bool {
        *self == Tile::Void
    }
    fn is_wall(&self) -> bool {
        *self == Tile::Wall
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => f.write_str(" "),
            Self::Wall => f.write_str("#"),
            Self::Floor => f.write_str("."),
        }
    }
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        match value {
            "." => Tile::Floor,
            " " => Tile::Void,
            "#" => Tile::Wall,
            _ => panic!("Error parsing Tile, cannot parse: {value}"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    D,
    R,
    U,
    L,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match &self {
            Direction::D => 'D',
            Direction::L => 'L',
            Direction::U => 'U',
            Direction::R => 'R',
        })
    }
}

impl Add<Turn> for Direction {
    type Output = Direction;
    fn add(self, rhs: Turn) -> Self::Output {
        match rhs {
            Turn::R => match self {
                Direction::D => Direction::L,
                Direction::L => Direction::U,
                Direction::U => Direction::R,
                Direction::R => Direction::D,
            },
            Turn::L => match self {
                Direction::L => Direction::D,
                Direction::U => Direction::L,
                Direction::R => Direction::U,
                Direction::D => Direction::R,
            },
        }
    }
}
impl AddAssign<Turn> for Direction {
    fn add_assign(&mut self, rhs: Turn) {
        *self = match rhs {
            Turn::R => match self {
                Direction::D => Direction::L,
                Direction::L => Direction::U,
                Direction::U => Direction::R,
                Direction::R => Direction::D,
            },
            Turn::L => match self {
                Direction::L => Direction::D,
                Direction::U => Direction::L,
                Direction::R => Direction::U,
                Direction::D => Direction::R,
            },
        }
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Self::R => 0,
            Self::D => 1,
            Self::L => 2,
            Self::U => 3,
        }
    }
}

impl Direction {
    fn oppose(&self) -> Self {
        match self {
            Direction::U => Direction::D,
            Direction::R => Direction::L,
            Direction::D => Direction::U,
            Direction::L => Direction::R,
        }
    }
}

#[derive(Clone, Copy)]
enum Turn {
    R,
    L,
}

impl TryFrom<char> for Turn {
    type Error = std::io::ErrorKind;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Turn::R),
            'L' => Ok(Turn::L),
            _ => Err(std::io::ErrorKind::InvalidData),
        }
    }
}

impl Debug for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::L => f.write_str("L"),
            Self::R => f.write_str("R"),
        }
    }
}

#[derive(Debug)]
struct CycleVec<T> {
    _vec: Vec<T>,
}

impl<T> Deref for CycleVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self._vec
    }
}

impl<T> DerefMut for CycleVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._vec
    }
}

impl<T> From<Vec<T>> for CycleVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self { _vec: value }
    }
}

impl<T> CycleVec<T>
where
    T: TileTrait,
{
    fn get_non_void_ending_len(&self) -> usize {
        self.iter()
            .rposition(|t| !t.is_void())
            .expect("Failed rposition")
    }
    fn get_first(&self) -> usize {
        self.iter()
            .position(|t| !t.is_void())
            .expect("Failed finding valid tile")
    }
    fn go_left(&self, x: usize) -> usize {
        match x.checked_sub(1) {
            None => self.get_non_void_ending_len(),
            Some(xx) if xx < self.get_first() => self.get_non_void_ending_len(),
            Some(xx) => xx,
        }
    }
    fn go_right(&self, x: usize) -> usize {
        let xx = if x + 1 <= self.get_non_void_ending_len().min(self.len() - 1) {
            x + 1
        } else {
            self.get_first()
        };
        return xx;
    }
}

impl<T> CycleVec<CycleVec<T>>
where
    T: TileTrait + Copy + Debug,
{
    fn get_first(&self) -> (usize, usize) {
        (
            self.first()
                .expect("Called first on empty CycleVec")
                .get_first(),
            0,
        )
    }
    fn get_col(&self, x: usize) -> CycleVec<T> {
        let v: Vec<T> = self
            .iter()
            .filter_map(|cv| {
                if let Some(t) = cv.get(x) {
                    Some(*t)
                } else {
                    None
                }
            })
            .collect();
        CycleVec::from(v)
    }

    fn go_left(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (self.get(y).expect("Wrong y index").go_left(x), y)
    }
    fn go_right(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (self.get(y).expect("Wrong y index").go_right(x), y)
    }
    fn go_up(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (x, self.get_col(x).go_left(y))
    }
    fn go_down(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (x, self.get_col(x).go_right(y))
    }
}

struct Map {
    map: CycleVec<CycleVec<Tile>>,
    history: HashMap<(usize, usize), Direction>,
    instructions: Vec<(u32, Option<Turn>)>,
    p: (usize, usize),
}

impl Map {
    fn walk(&mut self, d: Direction) -> Result<(), ()> {
        let np = match d {
            Direction::D => self.map.go_down(self.p),
            Direction::L => self.map.go_left(self.p),
            Direction::R => self.map.go_right(self.p),
            Direction::U => self.map.go_up(self.p),
        };
        let nt = self.map[np.1][np.0];
        assert!(!nt.is_void());
        if nt.is_wall() {
            return Err(());
        }
        self.history.insert(self.p, d);
        self.p = np;
        return Ok(());
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.iter().enumerate().try_for_each(|(y, c)| {
            c.iter().enumerate().try_for_each(|(x, el)| {
                if (x, y) == self.p {
                    f.write_char('P')
                } else {
                    match self.history.get(&(x, y)) {
                        Some(&dir) => dir.fmt(f),
                        None => el.fmt(f),
                    }
                }
            })?;
            f.write_char('\n')?;
            Ok(())
        })?;
        f.write_char('\n')
        // self.instructions.iter().try_for_each(|(ic, i)| {
        //     f.write_fmt(format_args!("{ic}"))?;
        //     i.fmt(f)?;
        //     Ok(())
        // })?;
        // f.write_char('\n')
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        fn parse_tile(input: &str) -> IResult<&str, Tile> {
            map(alt((tag(" "), tag("."), tag("#"))), Tile::from)(input)
        }
        fn parse_map_line(input: &str) -> IResult<&str, CycleVec<Tile>> {
            map(many0(parse_tile), CycleVec::from)(input)
        }
        fn parse_map(input: &str) -> IResult<&str, CycleVec<CycleVec<Tile>>> {
            map(
                many1(terminated(parse_map_line, char('\n'))),
                CycleVec::from,
            )(input)
        }
        fn parse_instr(input: &str) -> IResult<&str, Vec<(u32, Option<Turn>)>> {
            many1(map(tuple((digit1, alpha0)), |(digits, turn)| {
                (
                    u32::from_str_radix(digits, 10).expect("Error parsing instr"),
                    match turn {
                        "" => None,
                        t => Some(Turn::try_from(t.chars().next().unwrap()).unwrap()),
                    },
                )
            }))(input.trim())
        }
        let sep = value.find("\n\n").unwrap();
        let split = value.split_at(sep + 1);
        let mut _map = match parse_map(split.0) {
            Ok((_, mut _map)) => _map,
            Err(_err) => {
                panic!("Failed parsing Map")
            }
        };
        let instr = parse_instr(split.1)
            .expect("Failed to parse instructions")
            .1;
        let p = _map.get_first();
        Map {
            instructions: instr,
            history: HashMap::new(),
            map: _map,
            p,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map: Map = Map::from(input);
    let mut dir = Direction::R;
    let instrs = map.instructions.clone();
    for (nd, turn) in instrs {
        for _ in 0..nd {
            match map.walk(dir) {
                Err(()) => {
                    // Hit a wall
                    break;
                }
                Ok(()) => (),
            }
        }
        if let Some(t) = turn {
            dir += t;
        }
    }
    let dir_v: usize = dir.into();
    Some(1000 * (map.p.1 + 1) + 4 * (map.p.0 + 1) + dir_v)
}

struct Grid<T> {
    _v: Vec<Vec<T>>,
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().try_for_each(|line| {
            line.iter().try_for_each(|t| t.fmt(f))?;
            f.write_char('\n')
        })
    }
}

impl Grid<Tile> {}

impl<T> Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self._v
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._v
    }
}

struct CubeFace {
    grid: Grid<Tile>,
    up: Option<(Weak<RefCell<Self>>, Direction)>,
    right: Option<(Weak<RefCell<Self>>, Direction)>,
    down: Option<(Weak<RefCell<Self>>, Direction)>,
    left: Option<(Weak<RefCell<Self>>, Direction)>,
}

impl CubeFace {
    fn width(&self) -> usize {
        self.grid.first().unwrap().len()
    }
    fn set_neigh(
        &mut self,
        up: (Weak<RefCell<Self>>, Direction),
        right: (Weak<RefCell<Self>>, Direction),
        down: (Weak<RefCell<Self>>, Direction),
        left: (Weak<RefCell<Self>>, Direction),
    ) {
        self.up = Some(up);
        self.right = Some(right);
        self.down = Some(down);
        self.left = Some(left);
    }

    fn go_dir(
        &self,
        (weak_self, x, y, dir): &(Weak<RefCell<Self>>, usize, usize, Direction),
    ) -> (Weak<RefCell<Self>>, usize, usize, Direction) {
        match dir {
            Direction::U => match y.checked_sub(1) {
                Some(new_y) => (weak_self.clone(), *x, new_y, *dir),
                None => {
                    let (next_up, incoming_dir) = self.up.as_ref().unwrap();
                    let new_coords = translate_coords(dir, incoming_dir, *x, *y, self.width());
                    (
                        next_up.clone(),
                        new_coords.0,
                        new_coords.1,
                        incoming_dir.oppose(),
                    )
                }
            },
            Direction::R => {
                if x + 1 < self.width() {
                    (weak_self.clone(), x + 1, *y, *dir)
                } else {
                    let (next_right, incoming_dir) = self.right.as_ref().unwrap();
                    let new_coords = translate_coords(dir, incoming_dir, *x, *y, self.width());
                    (
                        next_right.clone(),
                        new_coords.0,
                        new_coords.1,
                        incoming_dir.oppose(),
                    )
                }
            }
            Direction::D => {
                if y + 1 < self.width() {
                    (weak_self.clone(), *x, y + 1, *dir)
                } else {
                    let (next_down, incoming_dir) = self.down.as_ref().unwrap();
                    let new_coords = translate_coords(dir, incoming_dir, *x, *y, self.width());
                    (
                        next_down.clone(),
                        new_coords.0,
                        new_coords.1,
                        incoming_dir.oppose(),
                    )
                }
            }
            Direction::L => match x.checked_sub(1) {
                Some(new_x) => (weak_self.clone(), new_x, *y, *dir),
                None => {
                    let (next_left, incoming_dir) = self.left.as_ref().unwrap();
                    let new_coords = translate_coords(dir, incoming_dir, *x, *y, self.width());
                    (
                        next_left.clone(),
                        new_coords.0,
                        new_coords.1,
                        incoming_dir.oppose(),
                    )
                }
            },
        }
    }
}

fn translate_coords(
    moving_dir: &Direction,
    incoming_dir: &Direction,
    x: usize,
    y: usize,
    width: usize,
) -> (usize, usize) {
    let width_max_indx = width - 1;
    // ugly...
    match (moving_dir, incoming_dir) {
        (Direction::D, Direction::R) => (width_max_indx, x),
        (Direction::R, Direction::D) => (y, width_max_indx),

        // opposing v
        (Direction::D, Direction::U) => (x, 0),
        (Direction::U, Direction::D) => (x, width_max_indx),
        // opposing h
        (Direction::R, Direction::L) => (0, y),
        (Direction::L, Direction::R) => (width_max_indx, y),

        (Direction::L, Direction::U) => (y, 0),
        (Direction::U, Direction::L) => (0, x),

        (Direction::L, Direction::L) => (0, width_max_indx - y),
        (Direction::R, Direction::R) => (width_max_indx, width_max_indx - y),

        (_, _) => {
            println!("{moving_dir:?}{incoming_dir:?}");
            unreachable!();
        }
    }
}

impl From<(&Grid<Tile>, usize, usize, usize)> for CubeFace {
    fn from((super_grid, x, y, width): (&Grid<Tile>, usize, usize, usize)) -> Self {
        let mut grid: Grid<Tile> = Grid { _v: Vec::new() };
        // println!("width: {width}");
        super_grid[y..(y + width)].iter().for_each(|line| {
            grid._v
                .push(line[x..(x + width)].iter().map(|&t| t).collect())
        });

        // asset no void tile in grid
        // println!("{grid:?}");
        assert!(grid
            .iter()
            .find(|line| line.iter().find(|t| t.is_void()).is_some())
            .is_none());

        CubeFace {
            grid,
            up: None,
            right: None,
            down: None,
            left: None,
        }
    }
}

struct Cube {
    faces: [Rc<RefCell<CubeFace>>; 6],
    instr: Vec<(u32, Option<Turn>)>,
    pos: (Weak<RefCell<CubeFace>>, usize, usize, Direction),
    history: Vec<(usize, usize, Direction)>,
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        fn parse_tile(input: &str) -> IResult<&str, Tile> {
            map(alt((tag(" "), tag("."), tag("#"))), Tile::from)(input)
        }
        fn parse_line(input: &str) -> IResult<&str, Vec<Tile>> {
            many0(parse_tile)(input)
        }
        fn parse_flat_cube(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
            many1(terminated(parse_line, char('\n')))(input)
        }
        fn parse_instr(input: &str) -> IResult<&str, Vec<(u32, Option<Turn>)>> {
            many1(map(tuple((digit1, alpha0)), |(digits, turn)| {
                (
                    u32::from_str_radix(digits, 10).expect("Error parsing instr"),
                    match turn {
                        "" => None,
                        t => Some(Turn::try_from(t.chars().next().unwrap()).unwrap()),
                    },
                )
            }))(input.trim())
        }
        let sep = value.find("\n\n").unwrap();
        let split = value.split_at(sep + 1);
        let flat_cube = match parse_flat_cube(split.0) {
            Ok((_, mut _map)) => _map,
            Err(_err) => {
                panic!("Failed parsing Map")
            }
        };

        let cube_width = flat_cube
            .first()
            .expect("flat_cube shouldn't be empty vec")
            .iter()
            .position(|t| !t.is_void())
            .expect("Failed finding valid tile on first line");
        let super_grid = Grid { _v: flat_cube };

        /*
         *      12
         *      3
         *     45
         *     6
         */
        let [c0, c1, c2, c3, c4, c5] = (0..6)
            .map(|n| {
                Rc::new(RefCell::new(CubeFace::from((
                    &super_grid,
                    get_face_offset(n, cube_width).0,
                    get_face_offset(n, cube_width).1,
                    cube_width,
                ))))
            })
            .array_chunks::<6>()
            .next()
            .unwrap();

        c0.borrow_mut().set_neigh(
            (Rc::downgrade(&c5), Direction::L),
            (Rc::downgrade(&c1), Direction::L),
            (Rc::downgrade(&c2), Direction::U),
            (Rc::downgrade(&c3), Direction::L),
        );
        c1.borrow_mut().set_neigh(
            (Rc::downgrade(&c5), Direction::D),
            (Rc::downgrade(&c4), Direction::R),
            (Rc::downgrade(&c2), Direction::R),
            (Rc::downgrade(&c0), Direction::R),
        );
        c2.borrow_mut().set_neigh(
            (Rc::downgrade(&c0), Direction::D),
            (Rc::downgrade(&c1), Direction::D),
            (Rc::downgrade(&c4), Direction::U),
            (Rc::downgrade(&c3), Direction::U),
        );
        c3.borrow_mut().set_neigh(
            (Rc::downgrade(&c2), Direction::L),
            (Rc::downgrade(&c4), Direction::L),
            (Rc::downgrade(&c5), Direction::U),
            (Rc::downgrade(&c0), Direction::L),
        );
        c4.borrow_mut().set_neigh(
            (Rc::downgrade(&c2), Direction::D),
            (Rc::downgrade(&c1), Direction::R),
            (Rc::downgrade(&c5), Direction::R),
            (Rc::downgrade(&c3), Direction::R),
        );
        c5.borrow_mut().set_neigh(
            (Rc::downgrade(&c3), Direction::D),
            (Rc::downgrade(&c4), Direction::D),
            (Rc::downgrade(&c1), Direction::U),
            (Rc::downgrade(&c0), Direction::U),
        );
        let instr = parse_instr(split.1)
            .expect("Failed to parse instructions")
            .1;
        let pos = (Rc::downgrade(&c0), 0, 0, Direction::R);
        Cube {
            faces: [c0, c1, c2, c3, c4, c5],
            pos,
            instr,
            history: vec![(cube_width, 0, Direction::R)],
        }
    }
}

impl Into<char> for &Tile {
    fn into(self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Void => ' ',
        }
    }
}

impl Into<char> for &Direction {
    fn into(self) -> char {
        match self {
            Direction::U => '^',
            Direction::R => '>',
            Direction::D => 'v',
            Direction::L => '<',
        }
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.faces[0].try_borrow().unwrap().width();
        let mut lines = Vec::with_capacity(width * 4);
        let mut tpl_line = Vec::with_capacity(width * 4);
        for _ in 0..(width * 4) {
            tpl_line.push(' ');
        }
        for _ in 0..(width * 4) {
            lines.push(tpl_line.clone());
        }
        for (index, cube) in self.faces.iter().enumerate() {
            let (off_x, off_y) = get_face_offset(index, width);
            let cube = cube.try_borrow().unwrap();
            cube.grid.iter().enumerate().for_each(|(y, line)| {
                line.iter().enumerate().for_each(|(x, t)| {
                    lines[off_y + y][off_x + x] = t.into();
                });
            });
        }
        self.history
            .iter()
            .for_each(|(rx, ry, dir)| lines[*ry][*rx] = dir.into());
        lines.iter().try_for_each(|line| {
            line.iter().try_for_each(|&c| f.write_char(c))?;
            f.write_char('\n')
        })
    }
}

fn get_face_offset(n: usize, width: usize) -> (usize, usize) {
    if n == 0 {
        return (width, 0);
    } else if n == 1 {
        return (2 * width, 0);
    } else if n == 2 {
        return (width, width);
    } else if n == 3 {
        return (0, 2 * width);
    } else if n == 4 {
        return (width, 2 * width);
    } else if n == 5 {
        return (0, 3 * width);
    }
    unreachable!();
}

impl Cube {
    fn walk(&mut self) -> Result<(), ()> {
        let np = self
            .pos
            .0
            .upgrade()
            .unwrap()
            .try_borrow()
            .unwrap()
            .go_dir(&self.pos);
        let (nx, ny) = (np.1, np.2);
        let tile = np.0.upgrade().unwrap().try_borrow().unwrap().grid[ny][nx];
        assert!(!tile.is_void());
        if tile.is_wall() {
            return Err(());
        } else if tile.is_void() {
            panic!("In void");
        } else {
            self.pos = np;
            let real_coords = self.get_real_coords();
            self.history
                .push((real_coords.0, real_coords.1, self.pos.3))
        }
        Ok(())
    }

    fn print_real_cords(&self) {
        let real_coords = self.get_real_coords();
        println!(
            "c={} r={}, d={}",
            real_coords.0,
            real_coords.1,
            Into::<usize>::into(self.pos.3)
        );
    }
    fn turn(&mut self, turn: Turn) {
        self.pos = (
            self.pos.0.clone(),
            self.pos.1,
            self.pos.2,
            self.pos.3 + turn,
        );
    }
    fn get_face_index(&self, face_ref: Weak<RefCell<CubeFace>>) -> usize {
        (0..6)
            .find(|&n| Rc::ptr_eq(&face_ref.upgrade().unwrap(), &self.faces[n]))
            .unwrap()
    }
    fn get_real_coords(&self) -> (usize, usize) {
        let coords = (self.pos.1, self.pos.2);
        let width = self.faces[0].try_borrow().unwrap().width();

        let face_ref = self.pos.0.clone();
        let (offset_x, offset_y) = get_face_offset(self.get_face_index(face_ref), width);

        (coords.0 + offset_x, coords.1 + offset_y)
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cube = Cube::from(input);
    let instrs = cube.instr.clone();

    for (nd, turn) in instrs {
        for _ in 0..nd {
            match cube.walk() {
                Err(()) => {
                    // Hit a wall
                    break;
                }
                Ok(()) => (),
            }
        }
        if let Some(t) = turn {
            cube.turn(t);
        }
    }
    let dir_v: usize = cube.pos.3.into();
    let (x, y) = cube.get_real_coords();
    // println!("value is 1000 * (y:{y} + 1) + 4 * (x:{x} + 1) + {dir_v}");
    Some(1000 * (y + 1) + 4 * (x + 1) + dir_v)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("inputs", 22);
        // let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(108311));
    }
}

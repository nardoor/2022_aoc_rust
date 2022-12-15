// debug
// 🎄 Part 1 🎄
// 5513 (elapsed: 4.54ms)
// 🎄 Part 2 🎄
// 2427 (elapsed: 8.72ms)

// release
// 🎄 Part 1 🎄
// 5513 (elapsed: 187.23µs)
// 🎄 Part 2 🎄
// 2427 (elapsed: 347.03µs)

use std::cmp::Ordering;
use advent_of_code::helpers::{Point, ShiftedGrid};

#[derive(Clone, Copy)]
enum Dir {
    R,
    U,
    L,
    D,
}

struct Move(Dir, u8);

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match value.trim().split_once(' ') {
            Some(("R", n)) => Move(Dir::R, n.parse::<u8>().expect("Not a decimal.")),
            Some(("U", n)) => Move(Dir::U, n.parse::<u8>().expect("Not a decimal.")),
            Some(("L", n)) => Move(Dir::L, n.parse::<u8>().expect("Not a decimal.")),
            Some(("D", n)) => Move(Dir::D, n.parse::<u8>().expect("Not a decimal.")),
            _ => panic!("Failed parsing value: {value} to Move."),
        }
    }
}

impl From<Dir> for fn(&mut Point) -> () {
    fn from(val: Dir) -> Self {
        match val {
            Dir::R => Point::right,
            Dir::U => Point::up,
            Dir::L => Point::left,
            Dir::D => Point::down,
        }
    }
}

struct ParseMovesResult {
    pub moves: Vec<Move>,
    pub top_right: Point,
    pub bot_left: Point,
}

fn parse_moves(input: &str) -> ParseMovesResult {
    let mut cur_x: i32 = 0;
    let mut cur_y: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;

    let moves = input
        .lines()
        .map(|l| {
            let Move(d, n) = Move::from(l);
            match d {
                Dir::R => {
                    cur_x += n as i32;
                    max_x = max_x.max(cur_x);
                }
                Dir::U => {
                    cur_y += n as i32;
                    max_y = max_y.max(cur_y);
                }
                Dir::L => {
                    cur_x -= n as i32;
                    min_x = min_x.min(cur_x);
                }
                Dir::D => {
                    cur_y -= n as i32;
                    min_y = min_y.min(cur_y);
                }
            }
            // println!("cur_x: {cur_x}, cur_y: {cur_y}");
            Move(d, n)
        })
        .collect();
    // println!("bot_left: {min_x}, {min_y}");
    ParseMovesResult {
        moves,
        top_right: Point(max_x, max_y),
        bot_left: Point(min_x, min_y),
    }
}

trait Knot {
    fn right(&mut self);
    fn left(&mut self);
    fn up(&mut self);
    fn down(&mut self);
    fn is_touching(&self, point: &Self) -> bool;
    fn towards_vert(&mut self, point: &Self);
    fn towards_hori(&mut self, point: &Self);
    fn trail_towards(&mut self, point: &Self);
}

// struct Point(i32, i32);
impl Knot for Point {
    fn right(&mut self) {
        self.0 += 1;
    }
    fn left(&mut self) {
        self.0 -= 1;
    }
    fn up(&mut self) {
        self.1 += 1;
    }
    fn down(&mut self) {
        self.1 -= 1;
    }
    fn is_touching(&self, point: &Self) -> bool {
        if (point.x() - self.x()).abs() <= 1 && (point.y() - self.y()).abs() <= 1 {
            return true;
        }
        false
    }
    fn towards_vert(&mut self, point: &Self) {
        match point.y().cmp(&self.y()) {
            Ordering::Equal => {},
            Ordering::Greater => self.up(),
            Ordering::Less => self.down(),
        }
    }
    fn towards_hori(&mut self, point: &Self) {
        match point.x().cmp(&self.x()) {
            Ordering::Equal => {},
            Ordering::Greater => self.right(),
            Ordering::Less => self.left(),
        }
    }
    fn trail_towards(&mut self, point: &Self) {
        if self.is_touching(point) {
            return;
        }
        self.towards_hori(point);
        self.towards_vert(point);
    }
}

struct Rope<const N: usize> {
    knots: [Point; N],
}

impl<const N: usize> Rope<N> {
    fn new(o: Point) -> Self {
        let knots = [o; N];
        Rope { knots }
    }

    fn head_mut(&mut self) -> &mut Point {
        assert!(N > 0);
        self.knots.first_mut().unwrap()
    }

    fn tail(&self) -> &Point {
        self.knots.last().unwrap()
    }

    fn body_follow(&mut self) {
        // head is pulled
        for i in 1..N {
            let &before = self.knots.get(i - 1).unwrap();
            self.knots.get_mut(i).unwrap().trail_towards(&before);
        }
    }

    fn pull_head(&mut self, f: fn(&mut Point) -> ()) {
        f(self.head_mut());
        self.body_follow();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let parse_moves_result = parse_moves(input);
    let mut history = ShiftedGrid::<bool>::new(
        parse_moves_result.bot_left,
        parse_moves_result.top_right,
        false,
    );
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);

    parse_moves_result.moves.iter().for_each(|&Move(dir, n)| {
        for _ in 0..n {
            Into::<fn(&mut Point) -> ()>::into(dir)(&mut head);
            tail.trail_towards(&head);
            history.set(&tail, true);
        }
    });

    Some(
        history
            .vec
            .iter()
            .filter(|&e| *e)
            .count()
            .try_into()
            .expect("Failed to convert usize to u32"),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let parse_moves_result = parse_moves(input);
    let mut history = ShiftedGrid::<bool>::new(
        parse_moves_result.bot_left,
        parse_moves_result.top_right,
        false,
    );
    let mut rope: Rope<10> = Rope::new(Point(0, 0));

    parse_moves_result.moves.iter().for_each(|&Move(dir, n)| {
        for _ in 0..n {
            rope.pull_head(Into::<fn(&mut Point) -> ()>::into(dir));
            history.set(rope.tail(), true);
        }
    });

    Some(
        history
            .vec
            .iter()
            .filter(|&e| *e)
            .count()
            .try_into()
            .expect("Failed to convert usize to u32"),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trail_towards() {
        let test_data = [
            // tail, head, tail after trail_towards
            (Point(0, 0), Point(1, 1), Point(0, 0)),
            (Point(0, 0), Point(1, 2), Point(1, 1)),
            (Point(0, 0), Point(2, 0), Point(1, 0)),
            (Point(0, 0), Point(2, 1), Point(1, 1)),
            (Point(0, 0), Point(0, 1), Point(0, 0)),
            (Point(0, 0), Point(0, 0), Point(0, 0)),
            (Point(0, 0), Point(0, 2), Point(0, 1)),
            (Point(2, 2), Point(0, 0), Point(1, 1)),
        ];

        for td in test_data {
            let mut t1 = td.0;
            t1.trail_towards(&td.1);
            assert_eq!(t1, td.2);
        }
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}

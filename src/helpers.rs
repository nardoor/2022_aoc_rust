/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::cmp::Ordering;

pub trait Within {
    fn within(&self, other1: &Self, other2: &Self) -> bool;
}

impl<T> Within for T
where
    T: Ord,
{
    fn within(&self, other1: &Self, other2: &Self) -> bool {
        let ma = other2.max(other1);
        let mi = other2.min(other1);
        mi <= self && self <= ma
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x().cmp(&other.x()) {
            Ordering::Equal => self.y().cmp(&other.y()),
            any => any,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ShiftedGrid<T> {
    pub vec: Vec<T>,
    bot_left: Point,
    line_l: usize,
}

impl<T> ShiftedGrid<T>
where
    T: Copy,
{
    pub fn new(bot_left: Point, top_right: Point, default: T) -> Self {
        let line_l = top_right.x() - bot_left.x() + 1;
        assert!(line_l >= 0);
        let col_h = top_right.y() - bot_left.y() + 1;
        assert!(col_h >= 0);
        let capacity = (line_l * col_h) as usize;
        // println!("bot_left: {bot_left:?}");
        ShiftedGrid {
            bot_left,
            vec: vec![default; capacity],
            line_l: line_l as usize,
        }
    }

    pub fn shift_coords(&self, point: &Point) -> Point {
        // requesting bot_left => 0,0
        // requesting 0,0 => -botleft.x, -botleft.y
        let x = point.x() - self.bot_left.x();
        let y = point.y() - self.bot_left.y();
        assert!(x >= 0);
        assert!(y >= 0);
        Point(x, y)
    }

    pub fn set(&mut self, point: &Point, val: T) {
        let point = self.shift_coords(point);
        // println!("\tlen: {}", self.vec.len());
        self.vec[point.x() as usize + (point.y() as usize * self.line_l)] = val;
    }
}

#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

#[macro_export]
macro_rules! btree_set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = BTreeSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}
/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
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
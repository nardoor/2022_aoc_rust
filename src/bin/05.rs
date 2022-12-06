#![feature(iter_array_chunks)]

use std::fmt::{Display, Write};

enum CargoCraneModel {
    Crane9000,
    Crane9001,
}

struct Instruction {
    n: usize,
    src: usize,
    dst: usize,
}

impl Instruction {
    fn destructure(&self) -> (usize, usize, usize) {
        (self.n, self.src, self.dst)
    }
}

#[derive(Debug)]
struct CrateStack {
    crates: Vec<char>,
}

impl CrateStack {
    fn new() -> Self {
        CrateStack { crates: vec![] }
    }

    fn push(&mut self, c: char) {
        self.crates.push(c);
    }

    fn pushn(&mut self, mut sl: Vec<char>) {
        self.crates.append(&mut sl);
    }

    fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    fn popn(&mut self, n: usize) -> Option<Vec<char>> {
        let mut res = vec![];
        if n > self.len() {
            return None;
        }
        for _ in 0..n {
            res.push(self.pop().expect("Failed popn."));
        }
        res.reverse();
        Some(res)
    }

    fn push_back(&mut self, c: char) {
        self.crates.insert(0, c);
    }

    fn peek(&self) -> Option<&char> {
        self.crates.last()
    }

    fn len(&self) -> usize {
        self.crates.len()
    }

    fn try_get(&self, i: usize) -> Option<&char> {
        self.crates.get(i)
    }
}

struct CargoCrane {
    model: CargoCraneModel,
    crates_parsed: bool,
    stacks: Vec<CrateStack>,
    instructions: Vec<Instruction>,
}

impl Display for CargoCrane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.stacks.iter().map(|s| s.len()).max().unwrap_or(0);
        for i in (0..height).rev() {
            self.stacks.iter().for_each(|s| {
                let c = s.try_get(i);
                match c {
                    None => f.write_str("    "),
                    Some(&c) => f.write_str(&format!("[{c}] ")),
                }
                .expect("Failed write_str in CargoCrane::Display.");
            });
            f.write_char('\n').ok();
        }
        for i in 0..self.stacks.len() {
            f.write_str(&format!(" {i}  "))
                .expect("Failed write_str in CargoCrane::Display");
        }
        Ok(())
    }
}

impl CargoCrane {
    fn new(lines: &str, model: CargoCraneModel) -> Self {
        let mut cargo_crane = CargoCrane {
            model,
            crates_parsed: false,
            instructions: vec![],
            stacks: vec![],
        };
        cargo_crane.parse_lines(lines);
        cargo_crane
    }

    fn parse_crates(&mut self, line: &str) {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                while i >= self.stacks.len() {
                    self.stacks.push(CrateStack::new());
                }
                if c != ' ' {
                    self.stacks[i].push_back(c);
                }
            });
    }

    fn parse_move(&mut self, line: &str) {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["move", n, "from", src, "to", dst] => self.instructions.insert(
                0,
                Instruction {
                    n: n.parse::<usize>()
                        .expect("Failed parsing int to Instruction."),
                    src: src
                        .parse::<usize>()
                        .expect("Failed parsing int to Instruction.")
                        - 1,
                    dst: dst
                        .parse::<usize>()
                        .expect("Failed parsing int to Instruction.")
                        - 1,
                },
            ),
            _ => panic!("Failed parsing line: \"{line}\""),
        }
    }

    fn parse_lines(&mut self, lines: &str) {
        let lines = lines.lines();
        lines.for_each(|l: &str| {
            if l.split(' ').all(|c| "0123456789".contains(c)) || l.trim().is_empty() {
                self.crates_parsed = true;
            } else if !self.crates_parsed {
                self.parse_crates(l);
            } else {
                self.parse_move(l);
            }
        })
    }
    fn step_solve(&mut self) {
        let (n, src, dst) = self
            .instructions
            .pop()
            .expect("Failed to pop instruction")
            .destructure();
        // println!("move {n} from {src} to {dst}");

        match self.model {
            CargoCraneModel::Crane9000 => {
                for _ in 0..n {
                    let c = self
                        .stacks
                        .get_mut(src)
                        .expect("Failed to get src stack")
                        .pop()
                        .expect("Couldn't pop out of stack, might be empty");

                    self.stacks
                        .get_mut(dst)
                        .expect("Failed to get dst stack")
                        .push(c);
                }
            }
            CargoCraneModel::Crane9001 => {
                let v = self
                    .stacks
                    .get_mut(src)
                    .expect("Failed to get src stack")
                    .popn(n)
                    .expect("Couldn't popn out of stack, might be empty");
                self.stacks
                    .get_mut(dst)
                    .expect("Failed to get dst stack")
                    .pushn(v);
            }
        }
    }

    #[inline]
    fn is_solved(&self) -> bool {
        self.instructions.is_empty()
    }

    fn get_sol(&self) -> String {
        let mut sol = String::new();
        self.stacks
            .iter()
            .for_each(|s| sol.push(*s.peek().expect("Empty stack, can't get sol.")));
        sol
    }
}

// ~1.95ms
pub fn part_one(input: &str) -> Option<String> {
    let mut crane = CargoCrane::new(input, CargoCraneModel::Crane9000);

    while !crane.is_solved() {
        // println!("{}\n", crane);
        crane.step_solve();
        // println!("{}\n\n", crane);
    }

    Some(crane.get_sol())
}
// ~2.05ms
pub fn part_two(input: &str) -> Option<String> {
    let mut crane = CargoCrane::new(input, CargoCraneModel::Crane9001);

    while !crane.is_solved() {
        // println!("{}\n", crane);
        crane.step_solve();
        // println!("{}\n\n", crane);
    }

    Some(crane.get_sol())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}

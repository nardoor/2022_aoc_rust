#![feature(iter_array_chunks)]

use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
enum Tree {
    Branch(Vec<Tree>),
    Leaf(u8),
}

impl Tree {
    fn parse_leaf(input: &str) -> IResult<&str, Self> {
        map(digit1, |digts: &str| {
            Tree::Leaf(digts.parse().expect("Failed parsing digits"))
        })(input)
    }
    fn parse_branch(input: &str) -> IResult<&str, Self> {
        map(
            delimited(
                tag("["),
                many0(preceded(
                    opt(tag(",")),
                    alt((Self::parse_leaf, Self::parse_branch)),
                )),
                tag("]"),
            ),
            Tree::Branch,
        )(input)
    }
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_branch, Self::parse_leaf))(input)
    }
    fn is_branch(&self) -> bool {
        match self {
            Tree::Branch(_) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Tree::Leaf(u1), Tree::Leaf(u2)) => u1.partial_cmp(u2),
            (Tree::Leaf(u1), b2) if b2.is_branch() => {
                Tree::Branch(vec![Tree::Leaf(*u1)]).partial_cmp(b2)
            }
            (b1, Tree::Leaf(u2)) if b1.is_branch() => {
                b1.partial_cmp(&Tree::Branch(vec![Tree::Leaf(*u2)]))
            }
            (Tree::Branch(v1), Tree::Branch(v2)) => {
                let mut v1_iter = v1.iter();
                let mut v2_iter = v2.iter();
                loop {
                    let (o_t1, o_t2) = (v1_iter.next(), v2_iter.next());
                    match (o_t1, o_t2) {
                        (Some(t1), Some(t2)) => match t1.partial_cmp(t2) {
                            Some(Ordering::Equal) => continue,
                            Some(o) => return Some(o),
                            None => unreachable!(), /* partial_cmp always returns something (nevre None) */
                        },
                        (Some(_t1), None) => return Some(Ordering::Greater),
                        (None, Some(_t2)) => return Some(Ordering::Less),
                        (None, None) => return Some(Ordering::Equal),
                    };
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl From<&str> for Tree {
    fn from(line: &str) -> Self {
        Self::parse(line).unwrap().1
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines: Vec<&str> = input.split('\n').collect();
    if lines.len() % 3 != 0 {
        lines.push("");
        assert_eq!(lines.len() % 3, 0);
    }
    /* let data: Vec<(Tree, Tree)> = */
    Some(
        lines
            .into_iter()
            .array_chunks::<3>()
            .map(|[l1, l2, _l3]| (Tree::from(l1), Tree::from(l2)))
            .enumerate()
            .filter(|(_i, (t1, t2))| t1 < t2)
            .fold(0, |acc, (i, _t)| acc + i + 1),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    /*
    [[2]] div1
    [[6]] div2
    */
    let div1 = Tree::Branch(vec![Tree::Branch(vec![Tree::Leaf(2)])]);
    let div2 = Tree::Branch(vec![Tree::Branch(vec![Tree::Leaf(6)])]);

    let mut packets: Vec<Tree> = input
        .lines()
        .filter(|&l| !l.is_empty())
        .map(Tree::from)
        .collect();
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_by(|t1, t2| t1.partial_cmp(t2).unwrap());

    let mut ind_div1 = 0;
    let mut ind_div2 = 0;

    packets.iter().enumerate().for_each(|(i, t)| {
        if t.partial_cmp(&div1).unwrap() == Ordering::Equal {
            ind_div1 = i + 1;
        } else if t.partial_cmp(&div2).unwrap() == Ordering::Equal {
            ind_div2 = i + 1;
        }
    });
    Some(ind_div1 * ind_div2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}

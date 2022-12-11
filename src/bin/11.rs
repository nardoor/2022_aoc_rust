/*
-> Debug:
🎄 Part 1 🎄
108240 (elapsed: 589.90µs)
🎄 Part 2 🎄
25712998901 (elapsed: 146.86ms)

-> Release:
🎄 Part 1 🎄
108240 (elapsed: 35.30µs)
🎄 Part 2 🎄
25712998901 (elapsed: 9.15ms)
*/
use std::collections::VecDeque;

use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending},
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum Var {
    Old,
    Val(usize),
}

impl Var {
    fn val(&self) -> Option<usize> {
        match self {
            Var::Old => None,
            Var::Val(u) => Some(*u),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Mul(Var, Var),
    Sum(Var, Var),
}

impl Operation {
    fn cpt(&self, old: usize) -> usize {
        match self {
            Operation::Mul(v1, v2) => v1.val().unwrap_or(old) * v2.val().unwrap_or(old),
            Operation::Sum(v1, v2) => v1.val().unwrap_or(old) + v2.val().unwrap_or(old),
        }
    }
}

#[derive(Debug)]
struct Item {
    worry_level: usize,
}

#[derive(Debug)]
struct Test {
    divider: usize,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    amount_inspect: usize,
}

impl Monkey {
    fn inspect_item(&mut self, item: &mut Item, relief: bool, relief_modulus: usize) {
        // Monkey inspects item
        item.worry_level = self.operation.cpt(item.worry_level);
        if relief {
            item.worry_level = (item.worry_level as f64 / 3.).floor() as usize;
        } else {
            item.worry_level %= relief_modulus
        }
        self.amount_inspect += 1;
    }

    fn test_item(&mut self, item: &Item) -> usize {
        if item.worry_level % self.test.divider == 0 {
            return self.test.true_monkey;
        }
        self.test.false_monkey
    }

    fn step_play_turn(&mut self, relief: bool, relief_modulus: usize) -> Option<(usize, Item)> {
        if self.items.is_empty() {
            return None;
        }
        let mut item = self.items.pop_front().unwrap();

        self.inspect_item(&mut item, relief, relief_modulus);
        let monkey_id = self.test_item(&item);

        Some((monkey_id, item))
    }
}

/* Parse Utils */
fn tabbing(input: &str) -> IResult<&str, ()> {
    map(many0(alt((tag(" "), tag("\t")))), drop)(input)
}

/* Parse Item */
fn parse_item(input: &str) -> IResult<&str, Item> {
    preceded(
        alt((tag(" "), tag(", "))),
        map(digit1, |d: &str| Item {
            worry_level: d.parse::<usize>().expect("Failed parsing Item digit."),
        }),
    )(input)
}

fn parse_items(input: &str) -> IResult<&str, Vec<Item>> {
    preceded(tag("Starting items:"), many1(parse_item))(input)
}

/* Parse Operation */
fn parse_var(input: &str) -> IResult<&str, Var> {
    alt((
        map(tag("old"), |_| Var::Old),
        map(digit1, |d: &str| Var::Val(d.parse::<usize>().unwrap())),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    preceded(
        tag("Operation: new = "),
        map(
            tuple((
                parse_var,
                delimited(tag(" "), alt((tag("+"), tag("*"))), tag(" ")),
                parse_var,
            )),
            |(var1, operator, var2)| match operator {
                "*" => Operation::Mul(var1, var2),
                "+" => Operation::Sum(var1, var2),
                _ => unreachable!(),
            },
        ),
    )(input)
}

/* Parse Test */
fn parse_branch(input: &str) -> IResult<&str, (bool, usize)> {
    pair(
        preceded(
            tag("If "),
            map(alphanumeric1, |b| match b {
                "true" => true,
                "false" => false,
                _ => unreachable!(),
            }),
        ),
        preceded(
            tag(": throw to monkey "),
            map(digit1, |d: &str| d.parse::<usize>().unwrap()),
        ),
    )(input)
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (to_parse, (divider, (bool1, id1), (bool2, id2))) = tuple((
        delimited(
            tag("Test: divisible by "),
            map(digit1, |d: &str| d.parse::<usize>().unwrap()),
            line_ending,
        ),
        delimited(tabbing, parse_branch, line_ending),
        preceded(tabbing, parse_branch),
    ))(input)?;

    let false_monkey = if !bool1 {
        id1
    } else if !bool2 {
        id2
    } else {
        unreachable!()
    };
    let true_monkey = if bool1 {
        id1
    } else if bool2 {
        id2
    } else {
        unreachable!()
    };

    Ok((
        to_parse,
        Test {
            divider,
            false_monkey,
            true_monkey,
        },
    ))
}

/* Parse Monkey */
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            delimited(
                tag("Monkey "),
                map(digit1, |d: &str| d.parse::<usize>().unwrap()),
                pair(tag(":"), line_ending),
            ),
            delimited(tabbing, parse_items, line_ending),
            delimited(tabbing, parse_operation, line_ending),
            delimited(tabbing, parse_test, opt(line_ending)),
        )),
        |(id, items, operation, test)| Monkey {
            id,
            items: VecDeque::from(items),
            operation,
            test,
            amount_inspect: 0,
        },
    )(input)
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    many1(preceded(opt(pair(tabbing, line_ending)), parse_monkey))(input)
}

struct KeepAway {
    monkeys: Vec<Monkey>,
    relief: bool,
    fake_relief_divider: usize,
}

impl KeepAway {
    fn new(monkeys: Vec<Monkey>, relief: bool) -> Self {
        let fake_relief_divider = if !relief {
            monkeys.iter().fold(1, |acc, m| acc * m.test.divider)
        } else {
            0
        };

        KeepAway {
            monkeys,
            relief,
            fake_relief_divider,
        }
    }

    fn play_turn(&mut self, monkey_id: usize) {
        assert_eq!(monkey_id, self.monkeys[monkey_id].id);
        while let Some((target_monkey_id, item)) = self
            .monkeys
            .get_mut(monkey_id)
            .unwrap()
            .step_play_turn(self.relief, self.fake_relief_divider)
        {
            // println!(
            //     "\tItem with worry level {} is thrown to monkey {target_monkey_id}",
            //     item.worry_level
            // );
            self.monkeys[target_monkey_id].items.push_back(item)
        }
    }

    fn play_round(&mut self) {
        for id in 0..self.monkeys.len() {
            // println!("Monkey {id}:");
            self.play_turn(id);
        }
    }

    fn monkey_business(&self) -> usize {
        let mut inspect_amounts: Vec<usize> =
            self.monkeys.iter().map(|m| m.amount_inspect).collect();
        // for (index, ia) in inspect_amounts.iter().enumerate() {
        // println!("Monkey {index} => inspected {ia} items !");
        // }
        inspect_amounts.sort();
        assert!(inspect_amounts.len() >= 2);
        inspect_amounts.last().unwrap() * inspect_amounts[inspect_amounts.len() - 2]
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rest, monkeys) = parse_monkeys(input).expect("Failed parsing Monkeys.");
    assert_eq!(rest.trim(), "");
    let mut keep_away = KeepAway::new(monkeys, true);

    for _i in 0..20 {
        // println!("round {_i}:");
        keep_away.play_round();
        // println!("====");
    }
    Some(keep_away.monkey_business())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rest, monkeys) = parse_monkeys(input).expect("Failed parsing Monkeys.");
    assert_eq!(rest.trim(), "");
    let mut keep_away = KeepAway::new(monkeys, false);

    for _i in 0..10_000 {
        // if _i % 100 == 0 {
        //     println!("round {_i}:");
        // }
        keep_away.play_round();
        // println!("====");
    }
    Some(keep_away.monkey_business())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkeys() {
        let input = advent_of_code::read_file("examples", 11);
        let (rest, monkeys) = parse_monkeys(input.as_str()).expect("Failed to parse monkeys");
        println!("rest: {rest}");
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}

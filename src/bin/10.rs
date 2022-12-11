use std::str::Lines;

use nom::{branch::alt, bytes::complete::tag, IResult};

const CYCLE_PARTICULIAR_VALUES_COUNT: usize = 6;
const CYCLE_PARTICULIAR_VALUES: [usize; CYCLE_PARTICULIAR_VALUES_COUNT] =
    [20, 60, 100, 140, 180, 220];

#[derive(Debug)]
enum Instr {
    NOOP,
    ADDX(i8),
}

struct InstrParser {}
impl InstrParser {
    fn parse_noop(input: &str) -> IResult<&str, Instr> {
        let (input, _) = tag("noop")(input)?;
        Ok((input, Instr::NOOP))
    }

    fn parse_addx(input: &str) -> IResult<&str, Instr> {
        let (input, _) = tag("addx ")(input)?;
        Ok((
            "",
            Instr::ADDX(i8::from_str_radix(input, 10).expect("Failed parsing i8.")),
        ))
    }

    fn parse_instr(input: &str) -> IResult<&str, Instr> {
        alt((Self::parse_noop, Self::parse_addx))(input)
    }

    fn parse(input: Lines) -> Vec<Instr> {
        input
            .map(|l| Self::parse_instr(l).expect("Failed parsing instr").1)
            .collect()
    }
}

struct VCPU {
    x_reg: isize,
    cycle: usize,
    next_cycle_particuliar_value_index: Option<usize>,
    interesting_signal_strenghts: [Option<isize>; CYCLE_PARTICULIAR_VALUES_COUNT],
    crt: Option<CRT>,
}

impl VCPU {
    fn new() -> Self {
        VCPU {
            x_reg: 1,
            cycle: 0,
            next_cycle_particuliar_value_index: Some(0),
            interesting_signal_strenghts: [None; CYCLE_PARTICULIAR_VALUES_COUNT],
            crt: None,
        }
    }
    fn set_crt(&mut self) {
        self.crt = Some(CRT::new());
    }
    fn get_signal_strength(&self) -> isize {
        self.x_reg * self.cycle as isize
    }
    fn get_total_signal_strength(&self) -> isize {
        self.interesting_signal_strenghts
            .iter()
            .filter_map(|&e| e)
            .sum()
    }
    fn increase_cycle(&mut self) {
        if let Some(crt) = &mut self.crt {
            crt.draw_pixel(self.x_reg, self.cycle);
        }
        self.cycle += 1;
        if let Some(index) = self.next_cycle_particuliar_value_index {
            if self.cycle == CYCLE_PARTICULIAR_VALUES[index] {
                self.interesting_signal_strenghts[index] = Some(self.get_signal_strength());
                self.next_cycle_particuliar_value_index =
                    if index + 1 < CYCLE_PARTICULIAR_VALUES_COUNT {
                        Some(index + 1)
                    } else {
                        None
                    }
            }
        }
    }
    fn run_instr(&mut self, instr: &Instr) {
        match instr {
            Instr::NOOP => {
                self.increase_cycle();
            }
            Instr::ADDX(d) => {
                self.increase_cycle();
                self.increase_cycle();
                self.x_reg += *d as isize;
            }
        }
    }
    fn compute_display(&mut self, instrs: &Vec<Instr>) {
        if self.crt.is_none() {
            panic!("Called compute_display without CRT!!");
        }
        for instr in instrs {
            self.run_instr(instr);
        }
    }
}

struct CRT {
    screen: String,
}

impl CRT {
    fn new() -> Self {
        CRT {
            screen: String::new(),
        }
    }

    fn draw_pixel(&mut self, x_reg: isize, cycle: usize) {
        let npixel_index = cycle % 40;
        let x: usize = x_reg.clamp(0, 39) as usize;
        if npixel_index <= x + 1 && npixel_index >= x - 1 {
            self.screen.push('#');
        } else {
            self.screen.push('.')
        }
        if npixel_index == 39 {
            self.screen.push('\n');
        }
        println!("{}\n\n", self.screen);
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let instrs = InstrParser::parse(input.lines());
    let mut vcpu = VCPU::new();
    instrs.iter().for_each(|instr| vcpu.run_instr(instr));
    Some(vcpu.get_total_signal_strength())
}

pub fn part_two(input: &str) -> Option<String> {
    let instrs = InstrParser::parse(input.lines());
    let mut vcpu = VCPU::new();
    vcpu.set_crt();
    vcpu.compute_display(&instrs);
    Some(vcpu.crt.unwrap().screen)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                [
                    "##..##..##..##..##..##..##..##..##..##..",
                    "###...###...###...###...###...###...###.",
                    "####....####....####....####....####....",
                    "#####.....#####.....#####.....#####.....",
                    "######......######......######......####",
                    "#######.......#######.......#######.....\n",
                ]
                .join("\n")
            )
        );
    }
}

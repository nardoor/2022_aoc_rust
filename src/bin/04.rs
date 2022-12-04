// use eyes;
// const INPUT_TEMPLATE: &str = "{}-{},{}-{}";

struct Assignment(u32, u32);

impl Assignment {
    fn contains(&self, othr: &Self) -> bool {
        self.0 <= othr.0 && self.1 >= othr.1
    }
    fn overlaps(&self, othr: &Self) -> bool {
        othr.0 <= self.1 && othr.0 >= self.0 || self.0 <= othr.1 && self.0 >= othr.0
    }
}

pub fn parse_line(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .flat_map(|spl| spl.split('-'))
        .map(|n| n.parse::<u32>().expect("Failed to parse input"))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|&line| {
                // 5.15ms !
                // let Some((x1,y1 ,x2, y2)) = eyes::try_parse!(line, INPUT_TEMPLATE, u32, u32, u32, u32) else {
                //     panic!("Failed parsing input line");
                // };
                // 2.55ms !
                let Ok([x1, y1, x2, y2]) : Result<[u32;4], _> = parse_line(line).try_into() else {
                    panic!("Failed parsing input line");
                };
                let a1 = Assignment(x1, y1);
                let a2 = Assignment(x2, y2);
                a1.contains(&a2) || a2.contains(&a1)
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|&line| {
                let Ok([x1, y1, x2, y2]): Result<[u32;4], _> = parse_line(line).try_into() else {
                    panic!("Failed parsing input line");
                };
                let a1 = Assignment(x1, y1);
                let a2 = Assignment(x2, y2);
                a1.overlaps(&a2)
            })
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn test_overlaps_1() {
        // ##2345###
        // #####567#
        let a1 = Assignment(2, 5);
        let a2 = Assignment(5, 7);
        assert!(a1.overlaps(&a2));
    }

    #[test]
    fn test_overlaps_2() {
        // ###345###
        // #12######
        let a1 = Assignment(3, 5);
        let a2 = Assignment(1, 2);
        assert!(!a1.overlaps(&a2));
    }

    #[test]
    fn test_overlaps_3() {
        // #####567#
        // ##2345###
        let a1 = Assignment(5, 7);
        let a2 = Assignment(2, 5);
        assert!(a1.overlaps(&a2));
    }

    #[test]
    fn test_overlaps_4() {
        // #12######
        // ###345###
        let a1 = Assignment(1, 2);
        let a2 = Assignment(3, 5);
        assert!(!a1.overlaps(&a2));
    }

    #[test]
    fn test_overlaps_5() {
        // ##234567#
        // ###345###
        let a1 = Assignment(2, 7);
        let a2 = Assignment(3, 5);
        assert!(a1.overlaps(&a2));
    }

    #[test]
    fn test_overlaps_6() {
        // ###345###
        // ##234567#
        let a1 = Assignment(3, 5);
        let a2 = Assignment(2, 7);
        assert!(a1.overlaps(&a2));
    }
}

use std::slice::IterMut;

// ┌╼ 23:30:41 2022_aoc_rust git:(main) ✗
// └$ cargo solved 08
//     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//      Running `target/debug/08`
// 🎄 Part 1 🎄
// 1805 (elapsed: 3.39ms)
// 🎄 Part 2 🎄
// 444528 (elapsed: 110.21ms)

// ┌╼ 23:30:47 2022_aoc_rust git:(main) ✗
// └$ cargo solve 08
//     Finished release [optimized] target(s) in 0.01s
//      Running `target/release/08`
// 🎄 Part 1 🎄
// 1805 (elapsed: 147.10µs)
// 🎄 Part 2 🎄
// 444528 (elapsed: 1.57ms)

#[derive(Debug)]
struct Tree {
    height: u8,
    juged_visible: bool,
    // scenery: Option<u32>,
}

#[derive(Debug)]
struct TreeLine {
    trees: Vec<Tree>,
}

#[derive(Debug)]
struct Forest {
    tree_lines: Vec<TreeLine>,
    judged_visible: Option<u32>,
}

impl From<&str> for TreeLine {
    fn from(input: &str) -> Self {
        let trees: Vec<Tree> = input
            .chars()
            .map(|c| {
                let height = c.to_digit(10).expect("Not a digit..") as u8;
                Tree {
                    height,
                    juged_visible: false,
                    // scenery: None,
                }
            })
            .collect();

        TreeLine { trees }
    }
}

impl From<&str> for Forest {
    fn from(input: &str) -> Self {
        let tree_lines = input.lines().map(TreeLine::from).collect();

        Forest {
            tree_lines,
            judged_visible: None,
        }
    }
}

impl Forest {
    fn get_tree_mut(&mut self, x: usize, y: usize) -> &mut Tree {
        self.tree_lines
            .get_mut(y)
            .expect("Failed indexing Forest")
            .trees
            .get_mut(x)
            .expect("Failed indexing TreeLine")
    }
    fn get_row_len(&self) -> usize {
        self.tree_lines.first().expect("Empty forest").trees.len()
    }
    fn get_col_len(&self) -> usize {
        self.tree_lines.len()
    }
    fn iter_mut_line(&mut self, y: usize) -> IterMut<'_, Tree> {
        self.tree_lines
            .get_mut(y)
            .expect("Failed indexing Forest")
            .trees
            .iter_mut()
    }

    fn iter_mut_col(&mut self, x: usize) -> impl Iterator<Item = &mut Tree> + DoubleEndedIterator {
        self.tree_lines.iter_mut().map(move |tree_line| {
            tree_line
                .trees
                .get_mut(x)
                .expect("Failed indexing TreeLine")
        })
    }
    fn visible_trees(&mut self) -> u32 {
        if self.judged_visible.is_some() {
            return self.judged_visible.unwrap();
        }

        let mut count = 0;
        // by line
        for y in 0..self.get_col_len() {
            // ->
            count += count_visible_one_way(self.iter_mut_line(y));
            // <-
            count += count_visible_one_way(self.iter_mut_line(y).rev());
        }

        // by col
        for x in 0..self.get_row_len() {
            // ->
            count += count_visible_one_way(self.iter_mut_col(x));

            // <-
            count += count_visible_one_way(self.iter_mut_col(x).rev());
        }
        self.judged_visible = Some(count);
        count
    }

    fn get_best_scenery(&mut self) -> u32 {
        let mut best_scenery = 0;
        for x in 0..self.get_row_len() {
            for y in 0..self.get_col_len() {
                best_scenery = best_scenery.max(self.evaluate_scenery(x, y));
            }
        }
        best_scenery
    }

    fn evaluate_scenery(&mut self, x: usize, y: usize) -> u32 {
        // if self.get_tree_mut(x, y).scenery.is_some() {
        //     return self.get_tree_mut(x, y).scenery.unwrap();
        // }

        let height = self.get_tree_mut(x, y).height;

        let row_len = self.get_row_len();
        let col_len = self.get_col_len();

        let mut count_left = 0;
        let mut count_top = 0;
        let mut count_right = 0;
        let mut count_down = 0;

        // right
        self.iter_mut_line(y)
            .skip(x + 1)
            .try_for_each(|t: &mut Tree| {
                count_right += 1;
                if t.height >= height {
                    return Err(());
                }
                Ok(())
            })
            .ok();

        // left
        self.iter_mut_line(y)
            .rev()
            .skip(row_len - x)
            .try_for_each(|t: &mut Tree| {
                count_left += 1;
                if t.height >= height {
                    return Err(());
                }
                Ok(())
            })
            .ok();

        // down
        self.iter_mut_col(x)
            .skip(y + 1)
            .try_for_each(|t: &mut Tree| {
                count_down += 1;
                if t.height >= height {
                    return Err(());
                }
                Ok(())
            })
            .ok();

        // up
        self.iter_mut_col(x)
            .rev()
            .skip(col_len - y)
            .try_for_each(|t: &mut Tree| {
                count_top += 1;
                if t.height >= height {
                    return Err(());
                }
                Ok(())
            })
            .ok();

        let scenery = count_down * count_left * count_right * count_top;
        // println!("Tree at {x},{y} has down: {count_down}, right: {count_right}, top: {count_top}, left: {count_left} -> scenery: {scenery}");
        // self.get_tree_mut(x, y).scenery = Some(scenery);

        scenery
    }
}

fn count_visible_one_way<'a>(mut iter: impl Iterator<Item = &'a mut Tree>) -> u32 {
    // fn count_visible_one_way(&mut self) -> u32 {
    let first_tree = iter
        .next()
        .expect("ERROR: expected at least 1 elem in iter.");
    let mut max_height = first_tree.height;
    let mut count = {
        if !first_tree.juged_visible {
            first_tree.juged_visible = true;
            1
        } else {
            0
        }
    };
    for tree in iter {
        if tree.height > max_height {
            max_height = tree.height;
            if !tree.juged_visible {
                tree.juged_visible = true;
                count += 1;
            }
        }
    }

    count
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(Forest::from(input).visible_trees())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(Forest::from(input).get_best_scenery())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}

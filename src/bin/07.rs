/*
-> Debug:
🎄 Part 1 🎄
1306611 (elapsed: 2.31ms)
🎄 Part 2 🎄
13210366 (elapsed: 2.26ms)

-> Release:
🎄 Part 1 🎄
1306611 (elapsed: 247.47µs)
🎄 Part 2 🎄
13210366 (elapsed: 224.72µs)
*/

use std::collections::HashMap;

enum Cmd {
    Ls,
    Pushd(String),
    Popd,
}

struct File {
    size: usize,
}

struct Dir {
    files: HashMap<String, File>,
    dirs: HashMap<String, Dir>,
}

impl Dir {
    fn new() -> Self {
        Dir {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn total_size(&self) -> usize {
        self.files.iter().fold(0, |acc, (_key, f)| acc + f.size)
            + self
                .dirs
                .iter()
                .fold(0, |acc, (_key, d)| acc + d.total_size())
    }

    fn get_sub_dir(&mut self, dir_path: &Vec<String>) -> &mut Dir {
        let mut cur_dir = self;
        for p in dir_path {
            if cur_dir.dirs.contains_key(p) {
                cur_dir = cur_dir.dirs.get_mut(p).unwrap();
            }
        }
        cur_dir
    }

    fn add_file(&mut self, (fname, f): (String, File)) {
        self.files.insert(fname, f);
    }

    fn add_dir(&mut self, (dname, d): (String, Dir)) {
        self.dirs.insert(dname, d);
    }

    fn push_file(&mut self, dir_path: &Vec<String>, file: (String, File)) {
        let target_dir = self.get_sub_dir(dir_path);
        target_dir.add_file(file);
    }

    fn push_dir(&mut self, dir_path: &Vec<String>, dir: (String, Dir)) {
        let target_dir = self.get_sub_dir(dir_path);
        target_dir.add_dir(dir);
    }

    fn list_sub_dirs(&self) -> Vec<&Dir> {
        self.dirs
            .iter()
            .flat_map(|(_k, d)| {
                let mut subds = d.list_sub_dirs();
                subds.insert(0, d);
                subds
            })
            .collect()
    }
}

fn parse_command(cmd: &str) -> Cmd {
    match cmd.strip_prefix("$ ").unwrap().trim() {
        "ls" => Cmd::Ls,
        cmd => match cmd.split_once(' ') {
            Some(("cd", "..")) => Cmd::Popd,
            Some(("cd", d)) => Cmd::Pushd(String::from(d)),
            _t => {
                println!("{_t:?} ??");
                panic!("Failed parsing command !");
            }
        },
    }
}

fn parse_file(file: &str) -> (String, File) {
    if let Some((size_str, name)) = file.trim().split_once(' ') {
        return (
            String::from(name),
            File {
                size: size_str.parse::<usize>().unwrap(),
            },
        );
    }
    panic!("Failed to parse file!");
}

fn parse_dir(dir: &str) -> (String, Dir) {
    if let Some((_dir, name)) = dir.trim().split_once(' ') {
        return (String::from(name), Dir::new());
    }
    panic!("Failed to parse dir!")
}

struct CliState {
    path: Vec<String>,
    root_dir: Dir,
}

impl CliState {
    fn new() -> Self {
        CliState {
            path: Vec::new(),
            root_dir: Dir::new(),
        }
    }

    fn parse_cli_output(&mut self, input: &str) {
        input.lines().for_each(|l| {
            if l.starts_with('$') {
                let cmd = parse_command(l);
                match cmd {
                    Cmd::Ls => {}
                    Cmd::Popd => {
                        self.path.pop();
                    }
                    Cmd::Pushd(d) => self.path.push(d),
                }
            } else if l.starts_with("dir") {
                let dir = parse_dir(l);
                self.root_dir.push_dir(&self.path, dir);
            } else {
                let file = parse_file(l);
                self.root_dir.push_file(&self.path, file);
            }
        });
    }
}

const TOTAL_DISK_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

pub fn part_one(input: &str) -> Option<usize> {
    let mut cli_state = CliState::new();
    cli_state.parse_cli_output(input);
    let subdirs = cli_state.root_dir.list_sub_dirs();
    let sum_sub_100_000 = subdirs
        .into_iter()
        .filter_map(|d| {
            let s = d.total_size();
            if s <= 100_000 {
                return Some(s);
            }
            None
        })
        .sum();
    Some(sum_sub_100_000)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cli_state = CliState::new();
    cli_state.parse_cli_output(input);

    let missing_space = NEEDED_SPACE - (TOTAL_DISK_SPACE - cli_state.root_dir.total_size());
    let subds = cli_state.root_dir.list_sub_dirs();
    let mut sizes: Vec<usize> = subds.into_iter().filter_map(|d|{
        let s = d.total_size();
        if s <= missing_space {
            return None;
        }
        Some(s)
    }).collect();
    sizes.sort();
    Some(*sizes.first().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}

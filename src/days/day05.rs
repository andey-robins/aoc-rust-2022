use crate::{Solution, SolutionPair};
use std::{collections::VecDeque, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day05.txt").expect("Error loading file.");
    five(input, 9)
}

#[test]
fn test() {
    let input = read_to_string("input/tests/day05.txt").expect("Unable to open test file");
    let (p1, p2) = five(input, 3);
    assert_eq!(p1, Solution::Str("CMZ".to_string()));
    assert_eq!(p2, Solution::Str("MCD".to_string()));
}

#[derive(Debug, Clone, Copy)]
struct Command {
    count: u64,
    src: u64,
    dst: u64,
}

impl Command {
    // the 9000 must move boxes one at a time
    fn run_9000(self, towers: &mut Vec<VecDeque<char>>) -> &mut Vec<VecDeque<char>> {
        for _ in 0..self.count {
            let moving_crate = towers[self.src as usize].pop_back().unwrap();
            towers[self.dst as usize].push_back(moving_crate);
        }
        towers
    }

    // the 9001 is able to pick up multiple boxes at once, moving them as a stack
    fn run_9001(self, towers: &mut Vec<VecDeque<char>>) -> &mut Vec<VecDeque<char>> {
        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..self.count {
            let moving_crate = towers[self.src as usize].pop_back().unwrap();
            tmp.push(moving_crate);
        }
        for _ in 0..self.count {
            towers[self.dst as usize].push_back(tmp.pop().unwrap());
        }

        towers
    }
}

fn five(input: String, width: u64) -> SolutionPair {
    let (commands, mut towers) = parse_input(input, width);
    let mut towers2 = towers.clone();

    for command in commands {
        towers = command.run_9000(&mut towers).to_vec();
        towers2 = command.run_9001(&mut towers2).to_vec();
    }

    // read world state into solution strings
    let mut sol1: String = String::new();
    let mut sol2: String = String::new();
    for tower in towers {
        sol1.push_str(tower[tower.len() - 1].to_string().as_str());
    }
    for tower in towers2 {
        sol2.push_str(tower[tower.len() - 1].to_string().as_str());
    }
    (Solution::Str(sol1), Solution::Str(sol2))
}

fn parse_input(input: String, width: u64) -> (Vec<Command>, Vec<VecDeque<char>>) {
    let mut towers: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..width {
        let tower: VecDeque<char> = VecDeque::new();
        towers.push(tower);
    }

    let mut sections = input.split("\n\n").collect::<Vec<&str>>();
    let code = sections.pop().unwrap();
    let raw_towers = sections.pop().unwrap();

    // process towers into vectors
    // parse towers
    let rows = raw_towers.split("\n").collect::<Vec<&str>>();
    for row in rows {
        if row.chars().nth(1).unwrap() == '1' {
            break;
        }

        for i in 0..width {
            let crate_label = row.chars().nth((i * 4 + 1) as usize).unwrap();
            if crate_label != ' ' {
                towers[i as usize].push_front(crate_label);
            }
        }
    }
    // towers now contains a vector of vecdeqs which are the towers of crates
    // process code into moves
    let raw_commands = code.split("\n").collect::<Vec<&str>>();
    let mut commands: Vec<Command> = Vec::new();

    for command in raw_commands {
        let tokens = command.split(" ").collect::<Vec<&str>>();
        let count: u64 = tokens[1].parse().unwrap();
        let src: u64 = tokens[3].parse().unwrap();
        let dst: u64 = tokens[5].parse().unwrap();

        commands.push(Command {
            count: count,
            src: src - 1,
            dst: dst - 1,
        });
    }

    (commands, towers)
}

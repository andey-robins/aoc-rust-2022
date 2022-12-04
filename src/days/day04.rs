use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04.txt").expect("Unable to read file.");
    four(input)
}

#[test]
fn test() {
    let input = read_to_string("input/tests/day04.txt").expect("Unable to read test file");
    let (p1, p2) = four(input);
    assert_eq!(p1, Solution::U64(2));
    assert_eq!(p2, Solution::U64(4));
}

fn four(input: String) -> SolutionPair {
    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let pair = line.split(",");
        let elves = pair.collect::<Vec<&str>>();
        let mut start_vals: Vec<u64> = Vec::new();
        let mut end_vals: Vec<u64> = Vec::new();

        for elf in elves {
            let mut range = elf.split("-").collect::<Vec<&str>>();
            let end: u64 = range.pop().unwrap().parse().unwrap();
            let start: u64 = range.pop().unwrap().parse().unwrap();
            start_vals.push(start);
            end_vals.push(end);
        }

        let elf2_start = start_vals.pop().unwrap();
        let elf1_start = start_vals.pop().unwrap();

        let elf2_end = end_vals.pop().unwrap();
        let elf1_end = end_vals.pop().unwrap();

        // part 1 solution calculation
        if elf1_start < elf2_start {
            if elf1_end >= elf2_end {
                sol1 += 1;
            }
        } else if elf1_start == elf2_start {
            sol1 += 1;
        } else {
            if elf1_end <= elf2_end {
                sol1 += 1;
            }
        }

        // part 2 solution calculation
        if elf1_start < elf2_start {
            if elf1_end >= elf2_start {
                sol2 += 1;
            }
        } else if elf1_start == elf2_start {
            sol2 += 1;
        } else if elf1_end == elf2_end {
            sol2 += 1;
        } else {
            if elf2_end >= elf1_start {
                sol2 += 1;
            }
        }
    }

    (Solution::U64(sol1), Solution::U64(sol2))
}

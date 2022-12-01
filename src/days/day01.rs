use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Elf {
    cals: u64,
}

impl Elf {
    fn add_cals(&mut self, k: u64) -> () {
        self.cals += k;
    }

    fn get_cals(&self) -> u64 {
        self.cals
    }
}

// impl PartialOrd for Elf {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         if self.cals >
//     }
// }

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").expect("Unable to read file");
    let mut lines = input.lines();

    let mut elves: Vec<Elf> = Vec::new();
    let mut elf: Elf = Elf { cals: 0 };

    while let Some(line) = lines.next() {
        if line == "" {
            elves.push(elf);
            elf = Elf { cals: 0 };
        } else {
            let cals: u64 = line.parse().unwrap();
            elf.add_cals(cals);
        }
    }

    elves.sort();
    elves.reverse();

    // Your solution here...
    let sol1: u64 = elves[0].get_cals();
    let sol2: u64 = elves[0].get_cals() + elves[1].get_cals() + elves[2].get_cals();

    assert!(sol1 == 67450);
    assert!(sol2 == 199357);

    (Solution::U64(sol1), Solution::U64(sol2))
}

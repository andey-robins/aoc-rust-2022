use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").expect("Unable to open file");
    three(input)
}

const ALPHABET: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[test]
fn test() {
    let input = read_to_string("input/tests/day03.txt").expect("Unable to open test file");
    let (p1, p2) = three(input);
    assert_eq!(p1, Solution::U64(157));
    assert_eq!(p2, Solution::U64(70));
}

fn three(input: String) -> SolutionPair {
    let mut lines = input.lines();
    let mut p1_total: u64 = 0;
    let mut p2_total: u64 = 0;

    let mut group: Vec<&str> = Vec::new();

    while let Some(line) = lines.next() {
        group.push(line);
        let first = &line[..line.chars().count() / 2];
        let second = &line[line.chars().count() / 2..];
        let mut mispacked: char = ' ';

        // calculate the part1 totals
        for chr in first.chars() {
            match second.find(chr) {
                Some(_) => mispacked = chr,
                None => (),
            }
        }
        p1_total += ALPHABET.find(mispacked).unwrap() as u64;

        // calculate part2 totals
        if group.len() == 3 {
            let mut common: char = ' ';
            let first = group.pop().unwrap();
            let second = group.pop().unwrap();
            let third = group.pop().unwrap();

            for chr in first.chars() {
                match (second.find(chr), third.find(chr)) {
                    (Some(_), Some(_)) => common = chr,
                    _ => (),
                }
            }

            p2_total += ALPHABET.find(common).unwrap() as u64;
            group = Vec::new();
        }
    }

    let sol1: u64 = p1_total;
    let sol2: u64 = p2_total;

    (Solution::U64(sol1), Solution::U64(sol2))
}

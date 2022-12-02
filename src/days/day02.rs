use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Result {
    Win,
    Lose,
    Draw,
}

fn parse_rps(choice: &str) -> RPS {
    match choice {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => unimplemented!(),
    }
}

fn parse_result(choice: &str) -> Result {
    match choice {
        "X" => Result::Lose,
        "Y" => Result::Draw,
        "Z" => Result::Win,
        _ => unimplemented!(),
    }
}

fn score_round(elf: RPS, player: RPS) -> u64 {
    match elf {
        RPS::Rock => match player {
            RPS::Rock => 4,
            RPS::Paper => 8,
            RPS::Scissors => 3,
        },
        RPS::Paper => match player {
            RPS::Rock => 1,
            RPS::Paper => 5,
            RPS::Scissors => 9,
        },
        RPS::Scissors => match player {
            RPS::Rock => 7,
            RPS::Paper => 2,
            RPS::Scissors => 6,
        },
    }
}

fn reverse_score_round(elf: RPS, result: Result) -> u64 {
    match elf {
        RPS::Rock => match result {
            Result::Win => 8,
            Result::Lose => 3,
            Result::Draw => 4,
        },
        RPS::Paper => match result {
            Result::Win => 9,
            Result::Lose => 1,
            Result::Draw => 5,
        },
        RPS::Scissors => match result {
            Result::Win => 7,
            Result::Lose => 2,
            Result::Draw => 6,
        },
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").expect("Unable to open file");
    two(input)
}

#[test]
pub fn test() {
    let input = read_to_string("input/tests/day02.txt").expect("Unable to open test file");
    let (sol1, sol2) = two(input);

    assert_eq!(sol1, Solution::U64(15));
    assert_eq!(sol2, Solution::U64(12));
}

pub fn two(input: String) -> SolutionPair {
    let mut lines = input.lines();

    let mut total = 0;
    let mut total2 = 0;
    while let Some(line) = lines.next() {
        let opponent = parse_rps(&line[0..1]);
        let player = parse_rps(&line[2..3]);
        let outcome = parse_result(&line[2..3]);
        let score = score_round(opponent, player);
        let score2 = reverse_score_round(opponent, outcome);
        total += score;
        total2 += score2;
    }

    let sol1: u64 = total;
    let sol2: u64 = total2;

    (Solution::U64(sol1), Solution::U64(sol2))
}

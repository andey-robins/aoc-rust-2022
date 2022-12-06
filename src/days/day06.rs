use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[test]
fn tests() {
    let in1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let in2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let in3 = "nppdvjthqldpwncqszvftbrmjlhg";
    let in4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let in5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    let idx1 = index_end_of_start_packet(in1, 4).unwrap();
    let idx2 = index_end_of_start_packet(in2, 4).unwrap();
    let idx3 = index_end_of_start_packet(in3, 4).unwrap();
    let idx4 = index_end_of_start_packet(in4, 4).unwrap();
    let idx5 = index_end_of_start_packet(in5, 4).unwrap();

    assert_eq!(idx1, 7);
    assert_eq!(idx2, 5);
    assert_eq!(idx3, 6);
    assert_eq!(idx4, 10);
    assert_eq!(idx5, 11);

    let idx1 = index_end_of_start_packet(in1, 14).unwrap();
    let idx2 = index_end_of_start_packet(in2, 14).unwrap();
    let idx3 = index_end_of_start_packet(in3, 14).unwrap();
    let idx4 = index_end_of_start_packet(in4, 14).unwrap();
    let idx5 = index_end_of_start_packet(in5, 14).unwrap();

    assert_eq!(idx1, 19);
    assert_eq!(idx2, 23);
    assert_eq!(idx3, 23);
    assert_eq!(idx4, 29);
    assert_eq!(idx5, 26);
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").expect("Unable to open input file");
    let sol1 = index_end_of_start_packet(input.as_str(), 4).unwrap();
    let sol2 = index_end_of_start_packet(input.as_str(), 14).unwrap();

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn index_end_of_start_packet(input: &str, width: u64) -> Option<u64> {
    for i in 0..input.len() {
        if no_dupes_in_substring(&input[i..i + width as usize], width) {
            return Some((i + width as usize) as u64);
        }
    }
    None
}

fn no_dupes_in_substring(string: &str, len: u64) -> bool {
    for i in 0..len {
        for j in i + 1..len {
            if string.chars().nth(i as usize) == string.chars().nth(j as usize) {
                return false;
            }
        }
    }
    true
}

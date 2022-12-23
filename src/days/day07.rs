use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;
use substring::Substring;

///////////////////////////////////////////////////////////////////////////////

const DISK_SIZE: u64 = 70_000_000;
const UPDATE_SIZE: u64 = 30_000_000;

#[test]
fn tests() {
    let example_input =
        read_to_string("input/tests/day07.txt").expect("Unable to read example input file");
    let (p1, p2) = day_seven(example_input);
    assert_eq!(p1, 95441);
    assert_eq!(p2, 24933642);
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let (sol1, sol2) =
        day_seven(read_to_string("input/day07.txt").expect("Unable to open input file for day 7"));

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn day_seven(input: String) -> (u64, u64) {
    let mut path = vec![];
    let mut directories: HashMap<String, u64> = HashMap::new();

    // populate initial directory sizes (this doesn't count the size of a nested directory)
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd ") {
            let next_dir = line.substring(5, line.len());
            if next_dir == ".." {
                path.pop();
            } else if next_dir == "/" {
                path = vec![];
            } else {
                path.push(next_dir);
            }
        } else if !line.starts_with("dir ") && !line.starts_with("$") {
            let size: u64 = line
                .split(" ")
                .nth(0)
                .expect("unable to get 0th element from split")
                .parse()
                .expect("unable to parse element");
            let working_dir = path_to_path_string(&path);
            directories
                .entry(working_dir)
                .and_modify(|curr_size| *curr_size += size)
                .or_insert(size);
        } else if !line.starts_with("$") {
            let working_dir = path_to_path_string(&path);
            directories
                .entry(working_dir)
                .and_modify(|curr_size| *curr_size = *curr_size)
                .or_insert(0);
        }
    }

    // cascade the nested directories up
    let mut all_paths: Vec<String> = directories.keys().cloned().collect();
    all_paths.sort_by(|a, b| a.len().cmp(&b.len()));
    all_paths.reverse();
    all_paths.pop();

    for curr_path in all_paths {
        let outer_size = *directories
            .get(&curr_path.to_string())
            .expect("expected directory value");
        match curr_path.rfind("/") {
            Some(idx) => {
                directories
                    .entry(curr_path.substring(0, idx).to_string())
                    .and_modify(|curr_size| *curr_size += outer_size)
                    .or_insert(outer_size);
            }
            None => {
                directories
                    .entry("".to_string())
                    .and_modify(|curr_size| *curr_size += outer_size);
            }
        }
    }

    let mut small_dir_sum: u64 = 0;
    for (_path, size) in &directories {
        if *size <= 100_000 {
            small_dir_sum += size;
        }
    }

    let root_size = *directories.get("").expect("failed to unwrap root size");
    // find the smallest directory we can delete and get enough space
    let free_space = DISK_SIZE - root_size;
    let needed_space = UPDATE_SIZE - free_space;
    let mut smallest_target_folder_size = root_size;
    for (_path, size) in &directories {
        if *size >= needed_space && *size < smallest_target_folder_size {
            smallest_target_folder_size = *size;
        }
    }
    (small_dir_sum, smallest_target_folder_size)
}

fn path_to_path_string(path: &Vec<&str>) -> String {
    path.join("/")
}

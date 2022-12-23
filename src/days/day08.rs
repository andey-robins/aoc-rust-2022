use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[test]
fn test() {
    let test_input = read_to_string("input/tests/day08.txt").expect("unable to open test file");
    let (p1, p2) = day_eight(test_input);
    assert_eq!(p1, 21);
    assert_eq!(p2, 8);
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/day08.txt").expect("Unable to open day8 input");
    let (sol1, sol2) = day_eight(input);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn day_eight(input: String) -> (u64, u64) {
    // parse in datastructures
    let mut lines = input.lines();
    let mut rows: Vec<Vec<u8>> = vec![];
    while let Some(line) = lines.next() {
        let mut row: Vec<u8> = vec![];
        for digit in line.chars() {
            let number: u8 = digit.to_string().parse().expect("unable to parse number");
            row.push(number);
        }
        rows.push(row);
    }

    let mut visible: Vec<Vec<bool>> = vec![];
    for _ in 0..rows.len() {
        let row = vec![false; rows.len()];
        visible.push(row);
    }
    let mut scores: Vec<Vec<u64>> = vec![];
    for _ in 0..rows.len() {
        let row = vec![0; rows.len()];
        scores.push(row);
    }
    // end of datastructure setup

    // count visible trees
    for x in 0..rows.len() {
        for y in 0..rows.len() {
            // check if each tree is visible from its x, y coord
            if x == 0 || y == 0 || x == rows.len() - 1 || y == rows.len() - 1 {
                // edge tree
                visible[y][x] = true;
            } else {
                // internal trees
                // check visibility to left and right
                let this_tree = rows[y][x];
                visible[y][x] = true;
                let mut left = true;
                for x2 in 0..rows.len() {
                    if x == x2 {
                        left = false;
                    } else if left {
                        if rows[y][x2] >= this_tree {
                            visible[y][x] = false;
                        }
                    } else if !left && !visible[y][x] {
                        if rows[y][x2] >= this_tree {
                            break;
                        } else if rows[y][x2] < this_tree && x2 == rows.len() - 1 {
                            visible[y][x] = true;
                        }
                    }
                }

                // if it isn't visible left to right, check if its visible up and down
                if !visible[y][x] {
                    visible[y][x] = true;
                    // check visibility up and down
                    let mut up = true;
                    for y2 in 0..rows.len() {
                        if y == y2 {
                            up = false;
                        } else if up {
                            if rows[y2][x] >= this_tree {
                                visible[y][x] = false;
                            }
                        } else if !up && !visible[y][x] {
                            if rows[y2][x] >= this_tree {
                                break;
                            } else if rows[y2][x] < this_tree && y2 == rows.len() - 1 {
                                visible[y][x] = true;
                            }
                        }
                    }
                }
            }
        }
    }

    // calculate scenic score
    for x in 1..rows.len() - 1 {
        for y in 1..rows.len() - 1 {
            let this_tree = rows[y][x];
            let (mut l, mut r, mut u, mut d) = (0, 0, 0, 0);
            let mut blocked = false;
            for x2 in (0..x).rev() {
                if rows[y][x2] < this_tree && !blocked {
                    l += 1;
                } else if !blocked {
                    blocked = true;
                    l += 1;
                }
            }
            blocked = false;
            for x2 in x + 1..rows.len() {
                if rows[y][x2] < this_tree && !blocked {
                    r += 1;
                } else if !blocked {
                    blocked = true;
                    r += 1;
                }
            }
            blocked = false;
            for y2 in (0..y).rev() {
                if rows[y2][x] < this_tree && !blocked {
                    u += 1;
                } else if !blocked {
                    blocked = true;
                    u += 1;
                }
            }
            blocked = false;
            for y2 in y + 1..rows.len() {
                if rows[y2][x] < this_tree && !blocked {
                    d += 1;
                } else if !blocked {
                    blocked = true;
                    d += 1;
                }
            }

            scores[y][x] = l * r * u * d;
        }
    }

    // collect answer
    let mut visible_trees = 0;
    for row in visible {
        for col in row {
            if col {
                visible_trees += 1;
            }
        }
    }

    let mut max_score = 0;
    for row in scores {
        for col in row {
            if col > max_score {
                max_score = col;
            }
        }
    }

    (visible_trees, max_score)
}

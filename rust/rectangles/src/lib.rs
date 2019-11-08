type Point = (usize, usize);

use std::collections::HashSet;
pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0;
    }

    let mut corners: HashSet<Point> = HashSet::new();
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if &lines[i][j..=j] == "+" {
                corners.insert((i, j));
            }
        }
    }

    let found: Vec<(Point, Point)> = Vec::new();
    let mut count = 0;
    for i in corners.iter() {
        for j in corners.iter() {
            if !(found.contains(&(*i, *j))) {
                if j.0 > i.0 && j.1 > i.1 && is_rectancle(lines, i.0, i.1, j.0, j.1) {
                    count += 1;
                }
            }
        }
    }

    count
}
fn is_rectancle(lines: &[&str], i: usize, j: usize, m: usize, n: usize) -> bool {
    // diagnonal points should also be '+'
    if &lines[i][n..=n] != "+" || &lines[m][j..=j] != "+" {
        return false;
    }

    let is_valid_row = |a: &str| {
        if a != "+" && a != "-" {
            return false;
        }
        true
    };

    let is_valid_col = |a: &str| {
        if a != "+" && a != "|" {
            return false;
        }
        true
    };

    // valid characters are |
    for k in lines.iter().take(m).skip(i) {
        if !is_valid_col(&k[j..=j]) || !is_valid_col(&k[n..=n]) {
            return false;
        }
    }

    for k in j..n {
        if !is_valid_row(&lines[i][k..=k]) || !is_valid_row(&lines[m][k..=k]) {
            return false;
        }
    }

    true
}

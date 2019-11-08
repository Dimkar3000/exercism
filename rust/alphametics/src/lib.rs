use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
struct Node {
    pub c: char,
    pub n: u8,
}

impl Node {
    fn new(c: char, n: u8) -> Self {
        Self { c, n }
    }

    fn to_tuple(&self) -> (char, u8) {
        (self.c, self.n)
    }
}

#[inline(always)]
fn calc(nodes: &[Node], word: &str) -> i64 {
    let mut val = 0i64;
    let mut m = 1i64;
    for i in word.chars().rev() {
        if let Some(x) = nodes.iter().find(|&x| x.c == i) {
            val += m * i64::from(x.n);
            m *= 10
        }
    }
    val
}

#[inline(always)]
fn check(nodes: &mut Vec<Node>, words: &[String], result: &str) -> bool {
    let c3 = result.chars().next().unwrap();

    if nodes.iter().any(|x| x.c == c3 && x.n == 0) {
        return false;
    }

    let func = |x: &Node| {
        words
            .iter()
            .any(|c| c.chars().next().unwrap() == x.c && x.n == 0)
    };

    if nodes.iter().any(func) {
        return false;
    }
    let vals: i64 = words.iter().map(|x| calc(nodes, x)).sum();
    let res = calc(nodes, result);
    vals == res
}


#[inline(always)]
fn perm(mut nodes: &mut Vec<Node>, words: &[String], result: &str) -> bool {
    let mut used = [false; 10];
    perm_internal(&mut nodes, 0, words, result, &mut used)
}

fn perm_internal(
    mut nodes: &mut Vec<Node>,
    n: u8,
    words: &[String],
    result: &str,
    used: &mut [bool; 10],
) -> bool {
    if n == nodes.len() as u8 - 1 {
        for (i, u) in used.iter().enumerate().take(10) {
            if !u {
                nodes[n as usize].n = i as u8;

                if check(&mut nodes, words, result) {
                    println!("found it");
                    return true;
                }
            }
        }
        return false;
    }
    for i in 0..10 {
        if !used[i] {
            nodes[n as usize].n = i as u8;
            used[i] = true;
            if perm_internal(nodes, n + 1, words, result, used) {
                return true;
            }
            used[i] = false;
        }
    }
    false
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let word_reg = Regex::new(r"\w+").unwrap();
    let words = word_reg
        .captures_iter(input)
        .map(|mat| mat[0].to_string())
        .collect::<Vec<String>>();

    if words.len() <= 2 {
        return None;
    }

    let mut freq: HashMap<char, i32> = HashMap::new();
    for x in words.iter() {
        for w in x.chars() {
            *freq.entry(w).or_insert(0) += 1
        }
    }

    let mut nodes: Vec<Node> = Vec::new();
    for (key, _) in freq {
        nodes.push(Node::new(key, 0))
    }

    if perm(
        &mut nodes,
        &words[..words.len() - 1],
        &words[words.len() - 1],
    ) {
        return Some(nodes.into_iter().map(|x| x.to_tuple()).collect());
    }
    None
}

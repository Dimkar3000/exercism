use std::collections::HashSet;
use rayon::prelude::*;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    (3..n/3).into_par_iter().filter_map(|a| {
        let b = n /2 - a * n / (2 * (n - a));
        let c = match a >= b {
            true => return None,
            false => n - a - b
        };
        match a.pow(2) + b.pow(2) == c.pow(2){
            true => Some([a,b,c]),
            false => None
        } 
    }).collect::<HashSet<[u32;3]>>()
}
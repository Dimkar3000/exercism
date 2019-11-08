use itertools::Itertools;
use std::sync::mpsc;
use std::thread;

fn is_palindrome(n: u64) -> bool {
    let n = n.to_string();
    n.chars().into_iter().eq(n.chars().into_iter().rev())
}

const NTHREADS: u64 = 8;

pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut pool = Vec::new();
    let (sender, receiver) = mpsc::channel::<Palindrome>();

    for t in 0..NTHREADS {
        let tx = sender.clone();
        pool.push(Some(thread::spawn(move || {
            for i in (min + t..=max).step_by(NTHREADS as usize) {
                for j in i..=max {
                    let product = i * j;
                    if is_palindrome(product) {
                        tx.send(product).unwrap();
                    }
                }
            }
        })));
    }

    for handle in &mut pool {
        if let Some(thread) = handle.take() {
            thread.join().unwrap();
        }
    }
    drop(sender);
    receiver.iter().sorted().collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    if palindromes.is_empty() {
        None
    }
    else {
        Some(palindromes.into_iter().map(move |x| *x).min().unwrap())
    }
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    if palindromes.is_empty() {
        None
    }
    else {
        Some(palindromes.into_iter().map(move |x| *x).max().unwrap())
    }
}

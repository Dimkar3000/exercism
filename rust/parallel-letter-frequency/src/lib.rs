use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunks: Vec<Vec<String>> = input
        .chunks(worker_count)
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| line.to_string())
                .collect::<Vec<_>>()
        })
        .collect();

    let (tx, rx) = channel();
    let mut workers = 0;
    for chunk in chunks {
        workers += 1;
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(count_lines(&chunk)).unwrap();
        });
    }

    let mut results = HashMap::new();

    for _ in 0..workers {
        let result = rx.recv().unwrap();
        for (c, n) in result.into_iter() {
            *results.entry(c).or_insert(0) += n;
        }
    }
    results
}

fn count_lines(lines: &[String]) -> HashMap<char, usize> {
    let mut results = HashMap::new();
    for line in lines.iter() {
        for c in line
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_lowercase().next().unwrap())
        {
            *results.entry(c).or_insert(0) += 1;
        }
    }
    results
}

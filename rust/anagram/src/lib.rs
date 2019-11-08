use itertools::sorted;
use std::collections::HashSet;

fn is_anagram<'a>(source: &'a str, candidate: &'a str) -> bool {
    if source.len() != candidate.len() || source.to_lowercase() == candidate.to_lowercase() {
        return false;
    }

    let func = |source: &'a str| source.chars().map(|x| x.to_lowercase().to_string());

    let s = sorted(func(source));
    let c = sorted(func(candidate));

    s.zip(c).all(|(i, j)| i == j)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut results = HashSet::new();
    results.extend(possible_anagrams.iter().filter(|x| is_anagram(word, x)));
    results
}

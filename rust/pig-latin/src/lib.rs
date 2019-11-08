fn apply_vowls_rule(word: &str) -> Option<String> {
    let rules = vec!["a", "o", "e", "i", "u", "xr", "yt"];
    for part in &rules {
        if word.starts_with(part) {
            return Some(word.to_string() + "ay");
        }
    }
    None
}

fn apply_consonant_rule(word: &str) -> String {
    let rules = vec![
        (0, "ch"),
        (1, "qu"),
        (0, "qu"),
        (0, "sch"),
        (0, "thr"),
        (0, "th"),
        (0, "xr"),
        (0, "yt"),
        (0, "rh"),
    ];

    let func = move |(index, x): (usize, &str)| {
        if word[index..].starts_with(x) {
            let split = x.len() + index;
            Some(word[split..].to_string() + &word[..split] + "ay")
        } else {
            None
        }
    };

    match rules.into_iter().find_map(func) {
        Some(x) => x,
        None => word[1..].to_string() + &word[0..1] + "ay",
    }
}

fn translate_word(word: &str) -> String {
    match apply_vowls_rule(word) {
        Some(x) => x,
        None => apply_consonant_rule(word),
    }
}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| translate_word(word))
        .collect::<Vec<_>>()
        .join(" ")
}

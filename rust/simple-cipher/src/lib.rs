use rand::Rng;
pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    };
    let shift_fn = |ch, k| (ch + k - 2 * (b'a')) % 26 + (b'a');
    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(ch, k)| shift_fn(ch as u8, k as u8) as char)
            .collect(),
    )
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    };
    let shift_fn = |ch, k| (ch + 26 - k) % 26 + b'a';
    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(ch, k)| shift_fn(ch as u8, k as u8) as char)
            .collect(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key: String = (0..100)
        .map(|_| (rng.gen_range(0, 26) + b'a') as char)
        .collect();
    let cipher_text = encode(&key.as_str(), s);
    (key, cipher_text.unwrap())
}

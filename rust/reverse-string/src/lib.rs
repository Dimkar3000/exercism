pub fn reverse(a: &str) -> String {
    String::from(a.chars().rev().collect::<String>())
}
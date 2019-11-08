fn safe_double(digit: u32) -> u32 {
    let result = digit * 2;
    if result > 9 {
        result - 9
    } else {
        result
    }
}

pub fn is_valid(input: &str) -> bool {
    if input.trim().len() < 2 || input.chars().any(|x| !x.is_digit(10) && !x.is_whitespace()) {
        return false;
    }

    let result = input
        .chars()
        .rev()
        .filter_map(|x| x.to_digit(10))
        .enumerate()
        .fold(0, |acc, (i, x)| {
            if i % 2 != 0 {
                acc + safe_double(x)
            } else {
                acc + x
            }
        });
    result % 10 == 0
}

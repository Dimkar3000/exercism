pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut r = num;
    while r > 0 {
        digits.push(r % 10);
        r /= 10;
    }

    let n = digits.iter().fold(0,|acc, &i| acc + i.pow(digits.len() as u32));
    num == n
}

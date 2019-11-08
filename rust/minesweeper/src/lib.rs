pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let mut result: Vec<Vec<char>> = minefield.iter().map(|x| x.chars().collect()).collect();
    let col_len = result.len();
    let row_len = result[0].len();

    for i in 0..minefield.len() {
        for j in 0..minefield[0].len() {
            if result[i][j] == ' ' {
                let mut count: u32 = 0;
                if i > 0 && j > 0 && result[i - 1][j - 1] == '*' {
                    count += 1;
                }
                if i > 0 && result[i - 1][j] == '*' {
                    count += 1;
                }
                if i > 0 && j < row_len - 1 && result[i - 1][j + 1] == '*' {
                    count += 1;
                }
                if j > 0 && result[i][j - 1] == '*' {
                    count += 1;
                }
                if j < row_len - 1 && result[i][j + 1] == '*' {
                    count += 1;
                }
                if i < col_len - 1 && j > 0 && result[i + 1][j - 1] == '*' {
                    count += 1;
                }
                if i < col_len - 1 && result[i + 1][j] == '*' {
                    count += 1;
                }
                if i < col_len - 1 && j < row_len - 1 && result[i + 1][j + 1] == '*' {
                    count += 1;
                }
                if count != 0 {
                    result[i][j] = std::char::from_digit(count, 10).unwrap();
                }
            }
        }
    }
    result
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect()
}

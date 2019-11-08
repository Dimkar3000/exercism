const ROWSIZE: usize = 4;
const COLSIZE: usize = 3;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

fn convert_letter(input: String) -> char {
    let input: &str = &input;
    match input {
        " _ | ||_|   " => '0',
        "     |  |   " => '1',
        " _  _||_    " => '2',
        " _  _| _|   " => '3',
        "   |_|  |   " => '4',
        " _ |_  _|   " => '5',
        " _ |_ |_|   " => '6',
        " _   |  |   " => '7',
        " _ |_||_|   " => '8',
        " _ |_| _|   " => '9',
        _ => '?',
    }
}

fn convert_line(input: &[String]) -> Vec<String> {
    let d = input[0].len() / COLSIZE;
    let mut result = vec![String::default(); d];

    for (j, y) in result.iter_mut().enumerate().take(d) {
        for x in input {
            y.push_str(&x[j * COLSIZE..(j * COLSIZE) + COLSIZE]);
        }
    }
    result
}
pub fn convert(input: &str) -> Result<String, Error> {
    let rows: Vec<String> = input.split('\n').map(|x| x.into()).collect();

    if rows.len() % ROWSIZE != 0 {
        return Err(Error::InvalidRowCount(rows.len()));
    } else if let Some(x) = rows.iter().find(|x| x.len() % COLSIZE != 0) {
        return Err(Error::InvalidColumnCount(x.len()));
    }

    let mut lines: Vec<Vec<String>> =
        vec![vec![String::default(); rows[0].len() / COLSIZE]; rows.len() / ROWSIZE];
    for j in (0..rows.len() / ROWSIZE).cycle().take(rows.len()) {
        lines[j] = convert_line(&rows[j * ROWSIZE..j * ROWSIZE + ROWSIZE])
    }
    let result: Vec<String> = lines
        .into_iter()
        .map(|x| x.into_iter().map(convert_letter).collect::<String>())
        .collect();

    Ok(result.join(&",".to_owned()))
}

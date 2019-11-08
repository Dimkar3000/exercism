pub fn get_diamond(c: char) -> Vec<String> {
    let dif = (c as u8 - b'A') as usize;
    let size = 2 * dif + 1;
    let mut result = vec![vec![' '; size as usize]; size as usize];

    for i in 0..=dif {
        let c = (b'A' + i as u8) as char;
        result[i][size / 2 + i] = c;
        result[i][size / 2 - i] = c;
        result[size - 1 - i][size / 2 + i] = c;
        result[size - 1 - i][size / 2 - i] = c;
    }

    result
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect()
}

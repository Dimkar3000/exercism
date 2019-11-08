pub fn number(input:&str) -> Option<String>{
    if input.chars().any(|c| !(c.is_digit(10) || "-+() .".chars().any(|x| x==c)) ){
        return None;
    }
    let input:Vec<char> = input.chars().filter(|c| c.is_digit(10)).collect();

    if input.len() == 10  && input[0] > '1' && input[3] > '1'{
        return Some(input.iter().collect());
    }
    else if input.len() == 11 && input[0] == '1' && input[1] > '1' && input[4] > '1'{
        return Some(input[1..].iter().collect());
    }
    None
}
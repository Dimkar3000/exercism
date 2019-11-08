/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(_isbn: &str) -> bool {
    let mut offset:u32 = 10;
    let mut result = 0;
    for i in _isbn.chars(){
        if i == '-'{
            continue;
        }else if i == 'X'{
            result += offset * 10
        }else{
            match i.to_digit(10){
                Some(x) => {
                    result += offset * x;
                    offset -= 1;
                },
                None => return false,
            }
        }
    }

    if result % 11 == 0{
        return true;
    }
    false

}
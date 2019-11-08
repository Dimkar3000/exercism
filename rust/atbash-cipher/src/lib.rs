use std::ascii::AsciiExt;
pub fn encode(input:&str) -> String{
    let mut result = String::new();
    let mut l = 0;
    for i in input.chars(){
        if i.is_alphabetic() && i.is_ascii(){
            result.push(std::char::from_digit((45 - i.to_digit(36).unwrap()),36).unwrap());
        }else if i.is_numeric(){
            result.push(i);
        }
    }

    for i in 1..(result.len()/5)+1{
        if i*5+l == result.len() {continue;}
        result.insert(i*5+l,' ');
        l+=1;
    }
    result

}

pub fn decode(input:&str) -> String{
    let mut result = String::new();
    for i in input.chars(){
        if i.is_alphabetic() && i.is_ascii(){
            result.push(std::char::from_digit((45 - i.to_digit(36).unwrap()),36).unwrap());
        }else if i.is_numeric(){
            result.push(i);
        }
    }
    result
}
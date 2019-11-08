pub fn is_pangram(sentence:&str) -> bool{
    let mut array:[bool;26] = [false;26];
    
    for i in sentence.to_lowercase().chars(){
        if !i.is_alphabetic(){continue;}
        array[(i.to_digit(36).unwrap_or(10) - 10) as usize] = true;
    }

    for i in array.iter(){
        if !i{
            return false;
        }
    }
    true
}
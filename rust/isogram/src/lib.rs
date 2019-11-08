fn is_in(list:&Vec<char>,r:&char) -> bool{
    for i in list.iter(){
        if *i == *r && r.is_alphabetic(){
            return true;    
        }    
    }
    false
}

pub fn check(sentence:&str) -> bool{
    let mut collected:Vec<char> = Vec::new();
    let split:Vec<char> = sentence.to_lowercase().chars().collect();
    for i in split.iter(){
        if is_in(&collected,i)&& i.is_alphabetic(){
            return false;
        }else{
            collected.push(*i);
        }
    }
    true
}
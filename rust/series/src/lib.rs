pub fn series(_digits: &str, _len: usize) -> Vec<String> {
    let mut result:Vec<String> = Vec::new();
    if _len > _digits.len(){return r;}
    for i in 0.._digits.len()-_len+1{
        result.push(String::from(_digits.get(i..i+_len).unwrap()));
    }
    result
}
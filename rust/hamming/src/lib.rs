pub fn hamming_distance(first:&str,second:&str) -> Result<u32,&'static str>{
    if first.len() != second.len(){
        return Err("");
    }
    let f:Vec<char> = first.chars().collect();
    let s:Vec<char> = second.chars().collect();
    let mut count:u32=0;
    for i in 0..first.len(){
        if f[i] != s[i] {
            count += 1;
        }
    }
    Ok(count)
}
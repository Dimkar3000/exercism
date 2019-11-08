use std::collections::HashMap;
pub fn nucleotide_counts(code :&str) -> Result<HashMap<char,usize>, &str>{
    let mut h: HashMap<char,usize> = HashMap::new();
    for c in ['G','T','A','C'].iter(){
        match count(*c,code){
            Ok(x) => h.insert(*c,x),
            Err(x) => return Err(x),
        };
    }
    Ok(h)
}

pub fn count(c:char,dna:&str) -> Result<usize, &str>{
    match c{
        'G' | 'T' | 'A' | 'C' => (),
        _ => return Err(""),
    }
    let mut result:usize = 0;
    for i in dna.chars(){
        if i == c { result+= 1;}
        else if i != 'G' && i !='T' && i != 'A' && i != 'C'{
            return Err("");
        }
    }
    Ok(result)
}
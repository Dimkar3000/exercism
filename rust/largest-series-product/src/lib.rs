pub fn lsp(word:&str,s:usize) -> Result<u32,&str>{
    if word.len() < s {
        return Err("");
    }else if s == 0{
        return Ok(1);
    }

    let mut result:u32 = 0;
    for i in 0..(word.len() - s+1){
        match word[i..i+s].parse::<u32>(){
            Ok(_) => {
                let mut tmp:u32  = 1;
                for j in word[i..i+s].chars(){
                    tmp *= j.to_digit(10).unwrap();
                };
                if tmp > result{result = tmp;}
            },
            Err(_) => return Err(""),
        };
    }
    Ok(result)
}
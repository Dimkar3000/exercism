pub fn rotate(input:&str,key:u32)->String{
    input.chars().map(|x|   if x.is_alphabetic(){
                                let mut r = std::char::from_digit((x.to_digit(36).unwrap()-10+key)%26 + 10,36).unwrap();
                                if x.is_uppercase(){
                                    r =  r.to_uppercase().next().unwrap()
                                };
                                r
                            }
                            else{x}
                     ).collect::<String>()
}
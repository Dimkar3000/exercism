pub struct Luhn{
    data:String
}
impl Luhn{
    pub fn is_valid(&self) -> bool{
        let input:String = self.data.clone();
        if input.trim().len() < 2{return false;}
        let mut result = 0;
        let mut flag = false;
        for i in input.chars().rev(){
            match i.to_digit(10){
                Some(x) => {
                    result += match flag {
                        true => {
                            if 2*x > 9{2*x - 9}
                            else{2*x}
                        },
                        false => x,
                    };
                    flag = !flag;
                },
                None => {
                    if i.is_whitespace(){
                        continue;
                    }else{
                        return false;
                    }
                },
            }
        }
        if result %10 == 0{
            return true;
        }
        false


    }

}
impl From<usize> for Luhn{
    fn from(input:usize) -> Luhn{
        Luhn{data:String::from(input.to_string())}
    }
}
impl From<u8> for Luhn{
    fn from(input:u8) -> Luhn{
        Luhn{data:String::from(input.to_string())}
    }
}
impl From<u16> for Luhn{
    fn from(input:u16) -> Luhn{
        Luhn{data:String::from(input.to_string())}
    }
}
impl From<u32> for Luhn{
    fn from(input:u32) -> Luhn{
        Luhn{data:String::from(input.to_string())}
    }
}
impl From<u64> for Luhn{
    fn from(input:u64) -> Luhn{
        Luhn{data:String::from(input.to_string())}
    }
}
impl From<String> for Luhn{
    fn from(input:String) -> Luhn{
        Luhn{data:input}
    }
}
impl<'a> From<&'a str> for Luhn{
    fn from(input:& 'a str) -> Luhn{
        Luhn{data:String::from(input)}
    }
}
impl Into<String> for Luhn{
    fn into(self) -> String{
        self.data
    }
}
pub trait Lunh{
    fn valid_luhn(&self) -> bool;
}

impl Lunh for String{
    fn valid_luhn(&self) -> bool{
        let input:String = self.clone();
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

impl Lunh for str{
    fn valid_luhn(&self) -> bool{
        let result = String::from(self);
        result.valid_luhn()
    }
}
impl Lunh for u8{
    fn valid_luhn(&self) -> bool{
        let result = self.to_string();
        result.valid_luhn()
    }
}
impl Lunh for u16{
    fn valid_luhn(&self) -> bool{
        let result = self.to_string();
        result.valid_luhn()
    }
}
impl Lunh for u32{
    fn valid_luhn(&self) -> bool{
        let result = self.to_string();
        result.valid_luhn()
    }
}
impl Lunh for u64{
    fn valid_luhn(&self) -> bool{
        let result = self.to_string();
        result.valid_luhn()
    }
}
impl Lunh for usize{
    fn valid_luhn(&self) -> bool{
        let result = self.to_string();
        result.valid_luhn()
    }
}
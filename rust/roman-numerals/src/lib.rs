pub struct Roman{
    n:String
}
impl Roman{
    pub fn from(input:i32)->Roman{
        let mut input = input;
        let mut result=String::new();
        while input>0{
            if input >= 1000{
                result.push('M');
                input-=1000;
            }
            else if input >= 900{
                result.push_str("CM");
                input -=900;
            }
            else if input >= 500{
                result.push_str("D");
                input -=500;
            }
            else if input >= 400{
                result.push_str("CD");
                input -=400;
            }
            else if input >= 100{
                result.push_str("C");
                input -=100;
            }
            else if input >= 90{
                result.push_str("XC");
                input -=90;
            }
            else if input >= 50{
                result.push_str("L");
                input -=50;
            }
            else if input >= 40{
                result.push_str("XL");
                input -=40;
            }
            else if input >= 10{
                result.push_str("X");
                input -=10;
            }
            else if input >= 9{
                result.push_str("IX");
                input -=9;
            } 
            else if input >= 5{
                result.push_str("V");
                input -=5;
            }
            else if input >= 4{
                result.push_str("IV");
                input -=4;
            }
            else{
                result.push_str("I");
                input -=1;
            }
        }
            Roman{n:result}       
    }
}

impl ToString for Roman{
    fn to_string(&self) -> String{ String::from(self.n.clone())}
}
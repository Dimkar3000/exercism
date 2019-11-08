pub struct Brackets{
    b:String
 }
use std::collections::VecDeque;
impl Brackets {
    pub fn from(input:&str) -> Brackets{
        Brackets{b:String::from(input)}
    }

    pub fn are_balanced(&self) -> bool{
        let mut found:VecDeque<char> = VecDeque::new();
        for i in self.b.chars(){
            if !"{}[]()".chars().any(|c| c == i){
                continue;
            }
            if "({[".chars().any(|c| c==i){
                found.push_front(i);
                continue;
            }
            match found.pop_front(){
                Some(x) =>  {
                        if (x == '{' && i=='}') || (x == '[' && i==']') || (x == '(' && i == ')'){
                            continue;
                        }else {
                            return false;
                        }
                },
                None    => return false,
            }


        }
        if found.is_empty(){
            return true;
        }
        false
    }
}
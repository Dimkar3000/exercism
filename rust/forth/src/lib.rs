use std::collections::hash_map::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, Default)]
pub struct Forth {
    stk: Vec<Value>,
    words: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth::default()
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stk.clone().to_vec()
    }

    pub fn eval_word(&mut self, i: &str) -> ForthResult {
        if let Some(x) = self.words.get(&i.to_lowercase()) {
            for j in x.to_string().split(' ') {
                let r = self.eval_word(&j.to_lowercase());
                if r.is_err() {
                    return r;
                }
            }
        } else if let Ok(x) = i.to_string().parse::<Value>() {
            self.stk.push(x);
        } else if i == "+" {
            let p1 = self.stk.pop();
            let p2 = self.stk.pop();

            if p1.is_some() && p2.is_some() {
                self.stk.push(p1.unwrap() + p2.unwrap());
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if i == "-" {
            let p1 = self.stk.pop();
            let p2 = self.stk.pop();

            if p1.is_some() && p2.is_some() {
                self.stk.push(p2.unwrap() - p1.unwrap());
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if i == "*" {
            let p1 = self.stk.pop();
            let p2 = self.stk.pop();

            if p1.is_some() && p2.is_some() {
                self.stk.push(p1.unwrap() * p2.unwrap());
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if i == "/" {
            let p1 = self.stk.pop();
            let p2 = self.stk.pop();

            if p1.is_some() && p2.is_some() {
                if p1 == Some(0) {
                    return Err(Error::DivisionByZero);
                }
                self.stk.push(p2.unwrap() / p1.unwrap());
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if i.to_lowercase() == "dup".to_lowercase() {
            let p1 = self.stk.pop();

            if p1.is_some() {
                let p1 = p1.unwrap();
                self.stk.push(p1);
                self.stk.push(p1);
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if i.to_lowercase() == "drop".to_lowercase() {
            let p1 = self.stk.pop();

            if p1.is_none() {
                return Err(Error::StackUnderflow);
            }
        } else if i.to_lowercase() == "swap".to_lowercase() {
            let p1 = self.stk.pop();
            let p2 = self.stk.pop();

            if p1.is_some() && p2.is_some() {
                let p1 = p1.unwrap();
                let p2 = p2.unwrap();
                self.stk.push(p1);
                self.stk.push(p2);
            } else {
                return Err(Error::StackUnderflow);
            }
        } else if i.to_lowercase() == "over".to_lowercase() {
            let p1 = self.stk.pop();
            let p2 = self.stk.pop();

            if p1.is_some() && p2.is_some() {
                let p1 = p1.unwrap();
                let p2 = p2.unwrap();
                self.stk.push(p2);
                self.stk.push(p1);
                self.stk.push(p2);
            } else {
                return Err(Error::StackUnderflow);
            }
        } else {
            return Err(Error::UnknownWord);
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut sentence: Vec<String> = Vec::new();
        let mut on_sentence = false;
        let mut name: Option<String> = None;
        for i in input.split(' ') {
            if on_sentence {
                if i == ";" {
                    for x in sentence.iter_mut() {
                        if let Some(y) = self.words.get(x) {
                            *x = y.to_string()
                        }
                    }

                    self.words.insert(
                        name.unwrap().to_lowercase(),
                        sentence.join(&" ").to_lowercase(),
                    );

                    name = None;
                    on_sentence = false;
                    sentence = Vec::new();
                    continue;
                }

                if name.is_none() {
                    if i.to_string().parse::<Value>().is_ok() {
                        return Err(Error::InvalidWord);
                    }
                    name = Some(i.to_string());
                } else {
                    sentence.push(i.to_string());
                }
            } else if i == ":" {
                on_sentence = true;
            } else {
                let r = self.eval_word(i);
                if r.is_err() {
                    return r;
                }
            }
        }
        if on_sentence {
            return Err(Error::InvalidWord);
        }
        Ok(())
    }
}

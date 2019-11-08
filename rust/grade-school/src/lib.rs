use std::collections::HashMap;
pub struct School {
    data:HashMap<u32,Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School{data:HashMap::new()}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self.data.get_mut(&grade){
            Some(x) => {x.push(String::from(student));x.sort();return;},
            None    => (),
        }
        self.data.insert(grade,vec![String::from(student)]);
    }

    pub fn grades(&mut self) -> Vec<u32> {
        let mut result:Vec<u32> = Vec::new();
        for i in self.data.keys(){
            result.push(*i);
        }
        result.sort();
        result
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.data.get(&grade){
            Some(x) => {
                let mut r:Vec<String> = vec![];
                for i in x.iter().clone(){
                    r.push(i.to_string());
                                
            }return Some(r)},
            None    => return None,
        }
    }
}
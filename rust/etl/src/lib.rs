use std::collections::BTreeMap;
pub fn transform(input:&BTreeMap<i32,Vec<char>>) -> BTreeMap<char,i32>{
    let mut result: BTreeMap<char,i32> = BTreeMap::new();
    input.iter().for_each(|(x,y)| y.iter().for_each(|c| match result.insert((*c).to_lowercase().next().unwrap(),*x){_ => ()}));
    result
}
use std::collections::HashMap;
//
fn form(input:&str) -> String{
    let mut result = String::new();
    for i in input.chars(){
        if i.is_alphanumeric(){
            result.push(i);
        }
    }
    result
}
pub fn word_count(sen:&str) -> HashMap<String,u32>{
    let mut result:HashMap<String,u32> = HashMap::new();
    for i in sen.split(' ').collect::<Vec<&str>>().iter(){
        let counter = result.entry(form(*i).to_lowercase()).or_insert(0);
        *counter += 1;
    }
    result.remove("");  //Remove empty String entry
    result
}
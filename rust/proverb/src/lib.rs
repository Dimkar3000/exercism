pub fn build_proverb(list: Vec<&str>) -> String {
    if list.len() == 0{
        return String::new();
    }
    let mut result = String::new();
    for i in 0..list.len()-1{
        result.push_str(format!("For want of a {} the {} was lost.\n",list[i],list[i+1]).as_str());
    }
    result.push_str(format!("And all for the want of a {}.",list[0]).as_str());
    result
}
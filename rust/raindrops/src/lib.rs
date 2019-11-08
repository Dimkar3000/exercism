pub fn raindrops(n: usize) -> String {
    let mut r = String::new();
    if n%3 == 0{
        r.push_str("Pling");
    }
    if n%5 == 0{
        r.push_str("Plang");
    }
    if n%7 == 0{
        r.push_str("Plong");
    }
    if r.is_empty(){
        return String::from(n.to_string())
    }
    r

}
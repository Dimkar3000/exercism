pub fn encrypt(input: &str) -> String {
    let s:String = input.to_lowercase().chars().filter(|&c| c>='a' && c<='z' ).collect();
    let c:usize = (s.len() as f64).sqrt().ceil() as usize;
    
    let mut result = vec![String::new();c];
    s.char_indices().for_each(|i| result[i.0%c].push(i.1));
    result.join(" ")
}
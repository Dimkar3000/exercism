use std::collections::HashMap;
pub fn tally(input:&str) -> String{
    let mut result = String::from("Team                           | MP |  W |  D |  L |  P");
    // Zero lines input
    if input == "" {
        return result;
    }
    //Multiple line input
    let mut teams = HashMap::new();
    let all_data =  input.split('\n').map(|c| String::from(c)).collect::<Vec<String>>();
    for i in all_data.iter(){
        let data = i.split(';').map(|c| String::from(c)).collect::<Vec<String>>();
        if data[2] == "win" {
            {        
                let pre = teams.entry(data[0].clone()).or_insert((0,0,0,0,0));
                pre.0 += 1;
                pre.1 += 1;
                pre.4 += 3;
            }
            {
                let pre = teams.entry(data[1].clone()).or_insert((0,0,0,0,0));
                pre.0 += 1;
                pre.3 += 1;
            }
        }else if data[2] == "loss" {
            {
                let pre = teams.entry(data[1].clone()).or_insert((0,0,0,0,0));
                pre.0 += 1;
                pre.1 += 1;
                pre.4 += 3;
            }
            {
                let pre = teams.entry(data[0].clone()).or_insert((0,0,0,0,0));
                pre.0 += 1;
                pre.3 += 1;
            }
        }else {
            {
                let pre = teams.entry(data[0].clone()).or_insert((0,0,0,0,0));
                pre.0 += 1;
                pre.2 += 1;
                pre.4 += 1;
            }
            {
                let pre = teams.entry(data[1].clone()).or_insert((0,0,0,0,0));
                pre.0 += 1;
                pre.2 += 1;
                pre.4 += 1;
            }
        }   
    }
    let mut data:Vec<(&String,(_))> = teams.iter().collect();
    data.sort_by(|a,b| if (a.1).4 != (b.1).4 { ((a.1).4).cmp(&(b.1).4).reverse()} else {a.0.cmp(b.0)});
    for i in data.iter(){
        let key = i.0;
        let value = i.1;
        result.push('\n');
        result.push_str(&format!("{pad:<width$} |  {} |  {} |  {} |  {} |  {}",value.0,value.1,value.2,value.3,value.4,pad=key,width=30)[..]);
    }
    result
}
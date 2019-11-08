pub fn abbreviate(input:&str) -> String{
    let mut result = String::new();

    for i in input.split(|c| c == ' ' || c =='-'){

        let mut pre = '0';
        let mut camel = false;
        for j in i.chars(){
            if pre == '0'{
                result.push(j.to_uppercase().next().unwrap());
                pre = j;
                continue;
            }
            if camel && pre.is_uppercase(){
                result.push(pre);
            }
            if pre.is_uppercase() && j.is_lowercase(){
                camel = true;
            }
            

            pre = j
        }

    }
    result
}
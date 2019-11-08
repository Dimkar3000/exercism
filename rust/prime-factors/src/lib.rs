pub fn factors(n:u64) -> Vec<u64>{
    let mut result = vec![];
    let mut current = n;
    let mut i = 2;
    loop{
        if current == 1{
            break;
        }
        
        if current %i == 0{
            current /= i;
            result.push(i);
        }
        else{
            i+=1;
        }

    }
    result
}
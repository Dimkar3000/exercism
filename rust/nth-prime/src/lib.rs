pub fn nth(a : usize) -> Result<i32,i32>{
    if a < 1{
        return Err(0);
    }
    let mut result: Vec<i32> = vec![2];
    let mut inc = 1;

    let mut f:bool;
    while result.len() < a{
        inc += 2;
        f = false;
        for i in result.iter(){
            if inc % *i == 0 {
                f = true;
                break;   
            }
        }

        if f == false{
            result.push(inc);
        }
    }
    match result.get(a-1) {
        None => Err(0),
        Some(x) => Ok(*x),
        }
   
        
}
// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("");
    }
    let mut steps: u64 = 0;
    let mut current:u64=n;
    loop{
        if current == 1 {break;}
        steps += 1;
        if current % 2 == 0{
            current /= 2;
        }
        else{
            current = 3 * current + 1;
        }
         
        
    }
    Ok(steps)


}
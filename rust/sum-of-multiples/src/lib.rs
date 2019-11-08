pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result:u32=0;
    for i in 0..limit as usize{
        for j in factors.iter(){
            if i%(*j as usize)==0{
                result += i as u32;
                break;
            }
        }
    }
    result
}
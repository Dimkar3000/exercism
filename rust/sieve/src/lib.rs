pub fn primes_up_to(input:usize) -> Vec<usize>{
    let mut result = Vec::new();
    let mut p = vec![true;input+1];
    p[0] = false;
    p[1] = false;
    for i in 2..input+1{
        if p[i]{
            result.push(i);
            for j in i..input{
                if i*j > input{
                    break;
                }
                p[i*j]= false;
            }
        }
    }
    result

}
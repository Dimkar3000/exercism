pub fn square_of_sum(n: usize) -> usize {
    let mut result:usize = 0;
    for i in 0..n+1{
        result += i;
    }
    result * result
    
}

pub fn sum_of_squares(n: usize) -> usize {
    let mut result:usize = 0;
    for i in 0..n+1{
        result += i*i;
    }
    result
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
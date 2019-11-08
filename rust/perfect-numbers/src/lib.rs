#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}

pub fn classify(num: u64) -> Result<Classification, & 'static str> {
    if num == 0{
        return Err("Number must be positive");
    }

    let mut sum=0;
    for i in 1..num{
        if num % i == 0{
            sum += i;
        }
    }

    if num == sum { 
        return Ok(Classification::Perfect);
    }
    else if sum < num{
        return Ok(Classification::Deficient);
    }
    else{
        return Ok(Classification::Abundant);
    }
}
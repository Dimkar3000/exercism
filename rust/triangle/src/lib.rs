#[derive(Debug)]
pub struct Triangle{
    is_equilateral:bool,
    is_isosceles:bool,
    is_scalene:bool,
}

impl Triangle{
    pub fn build<T:Copy>(sides:[T;3])->Result<Triangle,String> where f64:std::convert::From<T>{
        let sides0:f64 = f64::from(sides[0]);
        let sides1:f64 = f64::from(sides[1]);
        let sides2:f64 = f64::from(sides[2]);
        if sides0 == 0f64  || sides1  == 0f64 || sides2 == 0f64 || 
                                           (sides0 + sides1 < sides2) || 
                                           (sides0 + sides2 < sides1) || 
                                           (sides1 + sides2 < sides0){
            return Err(String::new());
        }
        let is_q = (sides0 == sides1) && (sides0 == sides2);
        let is_i = (sides0 == sides1) || (sides0 == sides2) || (sides1 == sides2);
        let is_s = !is_i;
        Ok(Triangle{is_equilateral:is_q,is_isosceles:is_i,is_scalene:is_s})
    }

    pub fn is_equilateral(&self)->bool{self.is_equilateral}
    pub fn is_isosceles(&self)->bool{self.is_isosceles}
    pub fn is_scalene(&self)->bool{self.is_scalene}
}
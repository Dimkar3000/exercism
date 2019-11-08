pub struct BowlingGame{
    data:Vec<(u64,u64)>,
    state:bool,
    temp:u64,
    have_temp:bool,
}

impl BowlingGame{
    pub fn new() -> BowlingGame{
        BowlingGame{data:Vec::new(),state:true,temp:0,have_temp:false}
    }

    fn get_score(&self,index:usize) -> Option<u64>{
        let x = match self.data.get(index){
            Some(x) => x,
            None    => return None,
        };
        if x.0 == 10 && x.1 == 0{
            let y = match self.data.get(index+1){
                Some(y) => y,
                None    => return None,
            };
            if y.0 == 10{
                match self.data.get(index +2){
                    Some(z) =>  return Some(x.0 + y.0 + z.0),
                    None    => return None,
                }
            }
            return Some(10 + y.0 + y.1);
        
        }else if x.0 + x.1 == 10{
            match self.data.get(index+1){
                Some(y) => return Some(10+y.0),
                None    => return None,
            }
        }else if x.0 + x.1 < 10{
            return Some(x.0+x.1);
        }
        else{
            return None;
        }
    }

    pub fn roll(& mut self,input:u64)-> Result<(),()>{
        if input > 10 {
            self.state = false;
            return Err(());
        }
        if self.data.len() == 10  && ((self.data[9].0 + self.data[9].1< 10) || (self.have_temp && input + self.temp > 10 ) ){
            self.state = false;
            return Err(());
        }
       
        if input == 10{
            self.data.push((10,0));
            return Ok(());
        }
        if self.have_temp{
            if self.temp + input <= 10{
                self.data.push((self.temp,input));
                self.have_temp = false;
            }
            else{
                self.state = false;
                return Err(());
            }

        }
        else{
            self.temp = input;
            self.have_temp = true;
        }
        if self.data.len() == 10 && self.data[9].0 < 10 && self.data[9].0 + self.data[9].1 == 10 && self.have_temp{
            self.data.push((self.temp,0));
            self.have_temp = false;
        }
        Ok(())
    }

    pub fn score(&self) ->Result<u64,()>{
        if !self.state{
            return Err(());
        }
        let mut sc = 0;
        for i in 0..10{
            match self.get_score(i){
                Some(x) => sc += x,
                None    => return Err(()),
            }
        }
        Ok(sc)
    }
}
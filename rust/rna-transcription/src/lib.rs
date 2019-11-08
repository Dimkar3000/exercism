#[derive(Debug)]
pub struct DNA{
    t: String,
    q: bool
}

#[derive(Debug)]
pub struct RNA{
    t: String
}

impl DNA{
    pub fn new(input:&str) -> DNA{
        let result:String=input.chars().filter(|c| *c=='G' || *c=='C' || *c=='A' || *c=='T').collect();
        if result == String::from(input){DNA{t:result,q:true} } else {DNA{t:String::from(input),q:false} }
    }

    pub fn to_rna(&self) -> Result<RNA,String>{
        if self.q{
            return Ok(RNA{t:self.t.chars().map(|c| if c=='G'{'C'} else if  c=='C'{'G'} else if c=='A' {'U'}else {'A'}).collect()});
        }
        Err(String::from(""))
    }
}

impl PartialEq for DNA{
    fn eq(&self, other: &DNA) -> bool{
        self.t == other.t
    }
}
impl PartialEq for RNA{
    fn eq(&self, other: &RNA) -> bool{
        self.t == other.t
    }
}



impl RNA{
    pub fn new(input:&str) -> RNA{
        let result:String=input.chars().filter(|c| *c=='G' || *c=='C' || *c=='A' || *c=='U').collect();
        RNA{t:result}
    }

}
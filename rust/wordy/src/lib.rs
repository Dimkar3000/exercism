pub struct WordProblem{
    input     :Vec<i64>,
    operations:Vec<String>,
}

impl WordProblem{
    pub fn new(input:&str) ->Self{
        let input:String = input[..input.len()-1].chars().collect();
        let mut n = Vec::new();
        let mut op = Vec::new();
        let mut is_operation = false;
        for i in input.split(' '){
            match i.parse::<i64>(){
                Ok(x) => {
                    n.push(x);
                    is_operation = true;
                },
                Err(_)    => {
                    if is_operation{
                        op.push(String::from(i));
                        is_operation = false;
                    }
                },
            }
        }
        WordProblem{input:n,operations:op}
    }

    pub fn answer(&self) -> Result<i64,()>{
        if self.input.len() != self.operations.len() +1 {
            return Err(());
        }
        println!("{:?}\n{:?}",self.input,self.operations);

        let mut result = self.input[0];
        for i in 0..self.operations.len(){
            if self.operations[i] == "minus"{
                result -= self.input[i+1]
        }else if self.operations[i] == "plus"{
            result += self.input[i+1];
        }else if self.operations[i] == "multiplied"{
            result *= self.input[i+1];
        }else if self.operations[i] == "divided"{
            if self.input[i+1] == 0{
                return Err(());
            }
            result /= self.input[i+1];
        }else{
            return Err(());
        }
        }
        return Ok(result);
    }
}
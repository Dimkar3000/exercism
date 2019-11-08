#[derive(Debug,PartialEq)]
pub struct CustomSet<T> where T:PartialEq+std::cmp::Ord,Vec<T>:Clone{
    data:Vec<T>,
}

impl<T> CustomSet<T> where T:PartialEq+std::cmp::Ord,Vec<T>:Clone{
    pub fn new(input:Vec<T>) -> Self{
        let mut result = CustomSet{data:Vec::new()};
        for i in input.into_iter(){
            result.add(i);
        }
        result
    }

    pub fn add(&mut self,input:T){
        if !self.data.iter().any(|x| *x == input){
            self.data.push(input);
        }
        self.data.sort();
    }

    pub fn is_empty(&self) -> bool{
        self.data.len() == 0
    }

    pub fn contains(&self,input:&T) -> bool{
        self.data.iter().any(|c| c == input)
    }

    pub fn is_subset(&self,other:&CustomSet<T>) -> bool{
        if self.data.is_empty() && other.data.is_empty(){
            return true;
        }
        else if self.data.len() > other.data.len(){
            return false;
        }
        self.data.windows(other.data.len()).all(|c| c == &other.data[..])
    }

    pub fn is_disjoint(&self,other:&CustomSet<T>) -> bool{
        !self.data.iter().any(|c| other.contains(c))
    }

    pub fn intersection(&self,other:&CustomSet<T>) -> CustomSet<T>{
        CustomSet{data:self.data.clone().into_iter().filter(|c| other.contains(c)).collect::<Vec<T>>()}
    }

    pub fn difference(&self,other:&CustomSet<T>) -> CustomSet<T>{
        CustomSet{data:self.data.clone().into_iter().filter(|c| !other.contains(c)).collect::<Vec<T>>()}
    }
    pub fn union(&self,other:&CustomSet<T>) -> CustomSet<T>{
        let mut result = CustomSet::new(self.data.clone());
        for i in other.data.clone().into_iter(){
            result.add(i);
        }
        result
    }
}
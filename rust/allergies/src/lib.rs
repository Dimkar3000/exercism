#[derive(Debug,PartialEq)]
pub enum Allergen{
    Eggs,//1
    Peanuts,//2
    Shellfish,//4
    Strawberries,//8
    Tomatoes,//16
    Chocolate,//32
    Pollen,//64
    Cats,//128

}

pub struct Allergies{
    data:Vec<Allergen>,
}

impl Allergies {
    pub fn new(input:u16)->Self{
        let mut result = Vec::new();
        if input & 0b_0000_0001 != 0{
            result.push(Allergen::Eggs);
        }
        if input & 0b_0000_0010 != 0{
            result.push(Allergen::Peanuts);
        }
        if input & 0b_0000_0100 != 0{
            result.push(Allergen::Shellfish);
        }
        if input & 0b_0000_1000 != 0{
            result.push(Allergen::Strawberries);
        }
        if input & 0b_0001_0000 != 0{
            result.push(Allergen::Tomatoes);
        }
        if input & 0b_0010_0000 != 0 {
            result.push(Allergen::Chocolate);
        }
        if input & 0b_0100_0000 != 0 {
            result.push(Allergen::Pollen);
        }
        if input & 0b_1000_0000 != 0 {
            result.push(Allergen::Cats);
        }
        Allergies{data:result}

    }

    pub fn allergies(self) -> Vec<Allergen>{
        self.data
    }

    pub fn is_allergic_to(&self,test:&Allergen) -> bool{
        self.data.iter().any(|c| c == test)
    }

}
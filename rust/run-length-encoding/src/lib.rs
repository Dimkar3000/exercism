use std::str::FromStr;

pub fn encode(word:&str)->String{
    //If string is empty return an empty String
    if word.len() == 0{ return String::new() }

    /*
     * Proccessing Starts
     * result contains the final product
     *
     * Go through every character counting how many of the same you have already encountered
     * When you find a new character there are two options
     * 1) if you counted only 1 then print it
     * 2) Otherwise print the current character count followed by the number
     * Then repeat the proccess for the new character
     *
     */
    let mut result:String = String::new();
    let mut count:i32=0;
    let mut current = ' ';
    for i in word.chars(){
        if count == 0{
            current = i;
            count =1;
        }else if current == i{
            count += 1;
        }else{
            if count > 1{
                result.push_str(count.to_string().as_str());
            }
            result.push(current);
            current = i;
            count = 1;
        }

    }
    if count > 1{
        result.push_str(count.to_string().as_str());
    }
    result.push(current);

    result

}
pub fn decode(word:&str)->String{
    //If string is empty return an empty String
    if word.len() == 0{ return String::new() }
    
    /*
     * Proccessing Starts
     * result contains the final product
     *
     * Start by reading the number:
     * 1)Create a string that contains all the numerical characters you have encountered
     * 2)When you encounter a no numeric character stop
     * Then there are 2 options
     * 1) if the number was 1 then print the character
     * 2) else print the character as many time as the number suggest
     * Then repeat the proccess for the next ncharacter who should be a number
     */

    let mut result:String = String::new();
    let mut count:String = String::new();

    for i in word.chars(){
        if i.is_numeric(){
            count.push(i);
        }else{
            match i32::from_str(count.as_str()){
                Ok(x) =>{
                    for _ in 0..x{
                        result.push(i);
                    }
                },
                Err(_) =>{
                    result.push(i);
                },
            }
            count = String::new();
        }
    }
    result

}
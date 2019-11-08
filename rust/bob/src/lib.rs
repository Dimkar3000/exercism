pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }

    let mut found_small = false;
    let mut found_letter = false;
    for i in message.chars(){ 
        if i.is_alphabetic() {
            found_letter = true;
            if i.is_lowercase() {
                found_small = true;
            }
        }
    }

    if found_letter && !found_small{
        return "Whoa, chill out!";
    }else if message.trim().ends_with("?"){
        return "Sure.";
    }    
    else{
        return "Whatever.";
    }

}
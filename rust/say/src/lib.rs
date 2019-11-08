fn translate_hundreds(x:u64)->String{
    let ekato = x / 100;
    let dekades  = (x % 100) / 10;
    let monades = (x % 100) % 10;
    let mut result = String::new();

    if ekato > 0{
        match ekato{
            1 => result.push_str("one "),
            2 => result.push_str("two "),
            3 => result.push_str("three "),
            4 => result.push_str("four "),
            5 => result.push_str("five "),
            6 => result.push_str("six "),
            7 => result.push_str("seven "),
            8 => result.push_str("eight "),
            9 => result.push_str("nine "),
            _ => (),
        }
        result.push_str("hundred ");
    }

    if dekades > 1 {
        match dekades{
            2 => result.push_str("twenty"),
            3 => result.push_str("thirty"),
            4 => result.push_str("forty"),
            5 => result.push_str("fifty"),
            6 => result.push_str("sixty"),
            7 => result.push_str("seventy"),
            8 => result.push_str("eighty"),
            9 => result.push_str("ninety"),
            _ => (),
        }
    }else if dekades == 1{
        match monades{
            0 => result.push_str("ten"),
            1 => result.push_str("eleven"),
            2 => result.push_str("twelve"),
            3 => result.push_str("thirteen"),
            4 => result.push_str("fourteen"),
            5 => result.push_str("fifteen"),
            6 => result.push_str("sixteen"),
            7 => result.push_str("seventeen"),
            8 => result.push_str("eighteen"),
            9 => result.push_str("nineteen"),
            _ => (),
        }
        return result;
    }

    if monades > 0{
        if dekades > 0{
            result.push('-');
        }
        match monades{
            1 => result.push_str("one"),
            2 => result.push_str("two"),
            3 => result.push_str("three"),
            4 => result.push_str("four"),
            5 => result.push_str("five"),
            6 => result.push_str("six"),
            7 => result.push_str("seven"),
            8 => result.push_str("eight"),
            9 => result.push_str("nine"),
            _ => (),
        }
    }
    String::from(result.trim())
}

pub fn encode(input:u64) -> String{
    if input == 0 {
        return String::from("zero");
    }

    let mut number:u64 = input;
    let mut result = String::new();

    if number >= 1_000_000_000_000_000_000 {
        number /= 1_000_000_000_000_000_000;
        result.push_str(&translate_hundreds(number));
        result.push_str(" quintillion ");
        number = input % 1_000_000_000_000_000_000;
    }
    if number >= 1_000_000_000_000_000 {
        number /= 1_000_000_000_000_000;
        result.push_str(&translate_hundreds(number));
        result.push_str(" quadrillion ");
        number = input % 1_000_000_000_000_000;
    }
    if number >= 1_000_000_000_000 {
        number /= 1_000_000_000_000;
        result.push_str(&translate_hundreds(number));
        result.push_str(" trillion ");
        number = input % 1_000_000_000_000;
    }
    if number >= 1_000_000_000 {
        number /= 1_000_000_000;
        result.push_str(&translate_hundreds(number));
        result.push_str(" billion ");
        number = input % 1_000_000_000;
    }
    if number >= 1_000_000 {
        number /= 1_000_000;
        result.push_str(&translate_hundreds(number));
        result.push_str(" million ");
        number = input % 1_000_000;
    }
    if number >= 1_000 {
        number /= 1_000;
        result.push_str(&translate_hundreds(number));
        result.push_str(" thousand ");
        number = input % 1_000;
    }

    result.push_str(&translate_hundreds(number));
    String::from(result.trim())
}
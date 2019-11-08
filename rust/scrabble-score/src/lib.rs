pub fn score(input:&str) -> u32{
    let mut result:u32 =0;
    for i in input.to_uppercase().chars(){
        result += match i{
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T'   =>  1,
            'D' | 'G'                                                   =>  2,
            'B' | 'C' | 'M' | 'P'                                       =>  3,
            'F' | 'H' | 'V' | 'W' | 'Y'                                 =>  4,
            'K'                                                         =>  5,
            'J' | 'X'                                                   =>  8,
            'Q' | 'Z'                                                   => 10,
            _                                                           =>  0
        }
    }
    result
}
fn single_to_bytes(mut n:u32) -> Vec<u8>{
    let mut result = Vec::new();
    result.push((n&0x7F) as u8);
    n >>= 7;
    loop{
        if n == 0{
            result.reverse();
            return result;
        }

        result.push(0x80 | ((n & 0x7F) as u8));
        n >>= 7;
    }
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(move |i| single_to_bytes(*i)).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>,()> {
    let mut result= Vec::new();
    let mut c:u32 = 0;
    let mut more = false;

    for i in bytes.iter(){
        c |= u32::from(*i & 0x7F);

        if (i & 0x80) != 0{
            more = true;
            if ((0xFE << 25) & c) != 0 {
                return Err(());
            }
            c <<=7;

        }else{
            more = false;
            result.push(c);
            c = 0;
        }
    }
    return if more{Err(())} else {Ok(result)}
}
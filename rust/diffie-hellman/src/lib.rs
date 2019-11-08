pub fn private_key(_p: u64) -> u64 {
    // Fake crypto emulation :D
    let a:u64 = _p & 0b1;
    if a != 0{
        return 3;
    }
    2
}

pub fn public_key(_p: u64, _g: u64, _a: u64) -> u64 {
    let r:u64 = (_g % _p).pow(_a as u32) % _p;
    r
}

pub fn secret(_p: u64, _b_pub: u64, _a: u64) -> u64 {
    let r:u64 =( _b_pub % _p ).pow(_a as u32) % _p;
    r
}
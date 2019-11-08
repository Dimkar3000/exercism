#[derive(Default)]
pub struct Robot {
    name: String,
}

static ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn gen_name() -> String {
    use rand::seq::SliceRandom;
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let l1: char = *ALPHABET.choose(&mut rng).unwrap();
    let l2: char = *ALPHABET.choose(&mut rng).unwrap();
    let num: u16 = rng.gen_range(0, 999);

    format!("{}{}{:03}", l1, l2, num)
}

impl Robot {
    pub fn new() -> Self {
        Robot { name: gen_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = gen_name();
    }
}

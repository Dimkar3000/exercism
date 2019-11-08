pub struct RailFence {
    pub rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut result = Vec::new();
        for i in 0..self.rails {
            for (j, c) in text.chars().enumerate() {
                if j % (2 * self.rails - 2) == i
                    || j % (2 * self.rails - 2) == 2 * self.rails - 2 - i
                {
                    result.push(c);
                }
            }
        }
        result.iter().collect()
        // text.chars().step_by(self.rails)
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut out: Vec<char> = vec![Default::default(); cipher.len()];
        let mut it = cipher.chars();

        
        for i in 0..self.rails {
            out.iter_mut()
                .enumerate()
                .filter(|(j, _)| {
                    j % (2 * self.rails - 2) == i
                        || j % (2 * self.rails - 2) == 2 * self.rails - 2 - i
                })
                .for_each(|(_, x)| *x = it.next().unwrap())
        }
        out.into_iter().collect()
    }
}

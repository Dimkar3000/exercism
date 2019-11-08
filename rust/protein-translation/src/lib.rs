use itertools::Itertools;
use std::collections::HashMap;
pub struct CodonsInfo<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.data.get(codon) {
            Some(&x) => Some(x),
            None => None,
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let result: Vec<&'a str> = rna
            .chars()
            .chunks(3)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .filter_map(|codon| match self.data.get(&codon[..]) {
                Some(&x) => Some(x),
                None => None,
            })
            .collect();
        if result.is_empty() {
            return None;
        }

        let i = result
            .iter()
            .position(|x| x == &"stop codon")
            .unwrap_or_else(|| result.len());
        Some(result[..i].to_vec())
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut result = CodonsInfo {
        data: HashMap::new(),
    };
    for (i, x) in pairs {
        result.data.insert(i, x);
    }
    result
}

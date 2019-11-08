pub type Domino = (u8, u8);

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    for chain in chains(input[0..1].to_vec(), input[1..].to_vec()) {
        if chain.get(0).unwrap().0 == chain.last().unwrap().1 {
            return Some(chain);
        }
    }
    None
}

fn chains(chain: Vec<Domino>, pile: Vec<Domino>) -> Vec<Vec<Domino>> {
    if pile.is_empty() {
        return vec![chain];
    }
    let root = chain.last().unwrap().1;
    let mut out = Vec::new();
    for d in 0..pile.len() {
        let mut rest = pile.clone();
        let mut domino = rest.remove(d);
        if root == domino.1 {
            domino = (domino.1, domino.0);
        } else if root != domino.0 {
            continue;
        }
        let mut new_chain = chain.clone();
        new_chain.push(domino);
        for chain in chains(new_chain, rest) {
            out.push(chain);
        }
    }
    out
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Sublist,
    Equal,
    Superlist,
    Unequal,
}

pub fn sublist<T>(a: &[T], b: &[T]) -> Comparison
where
    T: PartialEq,
{
    let x = |a: &[T], b: &[T]| -> bool { b.is_empty() || a.windows(b.len()).any(|c| c == b) };

    if a == b {
        Comparison::Equal
    } else if x(b, a) {
        Comparison::Sublist
    } else if x(a, b) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

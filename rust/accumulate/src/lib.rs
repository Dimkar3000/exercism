pub fn map_function<T, S>(items: Vec<T>, f: &Fn(T) -> S) -> Vec<S> {
    map_closure(items, f)
}
pub fn map_closure<T, S, F>(items: Vec<T>, mut f: F) -> Vec<S>  where F: FnMut(T) -> S,{
    items.into_iter().map(move |c| f(c)).collect()
}
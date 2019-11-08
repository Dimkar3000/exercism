// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?

pub struct Matcher<T> {
    function: Box<Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: AsRef<str>,
    {
        Matcher {
            function: Box::new(matcher),
            subs: subs.as_ref().to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
#[derive(Default)]
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Copy + ToString,
{
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        // self.matchers.push(matcher);
        let mut m = self.matchers;
        m.push(matcher);
        Fizzy { matchers: m }
    }

    /// map this fizzy onto every element of an interator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| self.apply_step(item))
    }

    fn apply_step(&self, item: T) -> String {
        let mut result = String::new();
        for matcher in self.matchers.iter() {
            if (matcher.function)(item) {
                result.push_str(&matcher.subs);
            }
        }
        if result.is_empty() {
            item.to_string()
        } else {
            result
        }
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Default + From<u8> + PartialEq + std::ops::Rem<Output = T> + 'static,
{
    let three: T = 3.into();
    let five: T = 5.into();
    Fizzy {
        matchers: vec![
            Matcher::new(move |n| n % three == T::default(), "fizz"),
            Matcher::new(move |n| n % five == T::default(), "buzz"),
        ],
    }
}

use std::collections::HashMap;
use std::hash::Hash;

pub struct Memoize<Closure, Arg, Result> {
    closure: Closure,
    results: HashMap<Arg, Result>,
}

impl<Closure, Arg, Result> Memoize<Closure, Arg, Result>
where
    Closure: Fn(Arg) -> Result,
    Arg: Eq + Hash + Copy,
    Result: Copy,
{
    pub fn new(closure: Closure) -> Memoize<Closure, Arg, Result> {
        Memoize {
            closure,
            results: HashMap::new(),
        }
    }

    pub fn call(&mut self, arg: Arg) -> Result {
        let res = self
            .results
            .entry(arg)
            .or_insert_with_key(|k| (self.closure)(*k));
        *res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn momoize_number() {
        let mut closure = Memoize::new(|x| x);
        assert_eq!(closure.call(2), 2);
        assert_eq!(closure.call(2), 2);
    }

    #[test]
    fn momoize_numbers() {
        let mut closure = Memoize::new(|x| x);
        assert_eq!(closure.call(2), 2);
        assert_eq!(closure.call(5), 5);
    }

    #[test]
    fn momoize_str() {
        let mut closure = Memoize::new(|x| x);
        assert_eq!(closure.call("foo"), "foo");
        assert_eq!(closure.call("bar"), "bar");
    }
}

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Memoize<Closure, Arg, Res> {
    closure: Closure,
    results: HashMap<Arg, Res>,
}

impl<Closure, Arg, Res> Memoize<Closure, Arg, Res>
where
    Closure: Fn(Arg) -> Res,
    Arg: Eq + Hash + Copy,
    Res: Copy,
{
    pub fn new(closure: Closure) -> Memoize<Closure, Arg, Res> {
        Memoize {
            closure,
            results: HashMap::new(),
        }
    }

    pub fn call(&mut self, arg: Arg) -> Res {
        match self.results.entry(arg) {
            Entry::Occupied(v) => *v.get(),
            Entry::Vacant(v) => (self.closure)(v.into_key()),
        }
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

pub struct Counter {
    count: usize,
    limit: usize,
}

impl Counter {
    pub fn new(limit: usize) -> Counter {
        Counter { count: 0, limit }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.limit {
            return None;
        }

        self.count += 1;
        Some(self.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_counter() {
        let counter = Counter::new(10);
        assert_eq!(0, counter.count);
        assert_eq!(10, counter.limit);
    }

    #[test]
    fn call_next() {
        let mut counter = Counter::new(3);
        assert_eq!(Some(1), counter.next());
        assert_eq!(Some(2), counter.next());
        assert_eq!(Some(3), counter.next());
        assert_eq!(None, counter.next());
    }

    #[test]
    fn iterate_over_counter() {
        let sum: usize = Counter::new(5)
            .zip(Counter::new(5).skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

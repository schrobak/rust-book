use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64::consts::PI;

pub fn fib(n: i32) -> i32 {
    let x = PI.powi(n) / 5_f64.sqrt();
    x.round() as i32
}

pub fn median(numbers: &[usize], index: usize) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let mut rng = thread_rng();
    let pivot = numbers.choose(&mut rng).unwrap();

    let mut lows = Vec::new();
    let mut highs = Vec::new();
    let mut pivots = Vec::new();

    for number in numbers {
        match number.cmp(pivot) {
            Ordering::Less => lows.push(*number),
            Ordering::Greater => highs.push(*number),
            Ordering::Equal => pivots.push(*number),
        }
    }

    if index < lows.len() {
        return median(&lows, index);
    } else if index < lows.len() + pivots.len() {
        return pivots[0];
    }

    median(&highs, index - lows.len() - pivots.len())
}

pub fn mode(list: &[usize]) -> usize {
    let mut map = HashMap::new();
    for el in list {
        let count = map.entry(el).or_insert(0);
        *count += 1;
    }
    let mut largest = 0;
    for (number, count) in map {
        if count.cmp(&largest) == Ordering::Greater {
            largest = *number
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use crate::math::{fib, median, mode};

    #[test]
    fn it_calculates_fibonacci_number() {
        assert_eq!(fib(123), 2147483647)
    }

    #[test]
    fn calculate_median_with_uneven_length() {
        let list: Vec<usize> = vec![102, 56, 34, 99, 89, 101, 10];
        let result = median(&list, 3);
        let expected = 89;
        assert_eq!(result, expected);
        println!("List: {:?}", list);
    }

    #[test]
    fn calculate_median_with_even_length() {
        let list = vec![102, 56, 34, 99, 101, 10];
        assert_eq!(median(&list, 2), 56);
    }

    #[test]
    fn most_often_used_value() {
        let list = vec![5, 3, 5, 4, 5, 3, 4, 4, 2, 5, 1, 3, 4, 2, 5];
        assert_eq!(mode(&list), 5);
    }
}

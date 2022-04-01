use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

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

#[cfg(test)]
mod tests {
    use crate::math::median;

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
}

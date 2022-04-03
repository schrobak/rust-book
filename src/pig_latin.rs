pub fn to_pig_latin(word: &str) -> String {
    let vowels = [b'a', b'e', b'i', b'o', b'u'];
    let word = String::from(word.trim()).to_lowercase();

    let word_bytes = word.as_bytes();
    let mut split_index = 0;

    for (index, byte) in word_bytes.iter().enumerate() {
        if vowels.contains(byte) {
            if index == 0 {
                return format!("{}-hey", word);
            }
            split_index = index;
        }
    }

    let (right, left) = word_bytes.split_at(split_index);

    format!(
        "{}-{}ay",
        String::from_utf8_lossy(left),
        String::from_utf8_lossy(right)
    )
}

#[cfg(test)]
mod tests {
    use crate::pig_latin::to_pig_latin;

    #[test]
    fn it_works_with_vowels() {
        assert_eq!("elephant-hey", to_pig_latin("elephant"))
    }

    #[test]
    fn it_works_with_consonant() {
        assert_eq!("og-day", to_pig_latin("dog"))
    }

    #[test]
    fn it_works_with_long_consonant() {
        assert_eq!("ush-bray", to_pig_latin("brush"))
    }
}

pub fn ttdoc() {
    let lines = [
        ("twelfth", "Twelve drummers drumming"),
        ("eleventh", "Eleven pipers piping"),
        ("tenth", "Ten lords a-leaping"),
        ("ninth", "Nine ladies dancing"),
        ("eight", "Eight maids a-milking"),
        ("seventh", "Seven swans a-swimming"),
        ("sixth", "Six geese a-laying"),
        ("fifth", "Five golden rings"),
        ("fourth", "Four calling birds"),
        ("third", "Three french hens"),
        ("second", "Two turtle doves, and"),
        ("first", "A partridge in a pear tree"),
    ];
    let len = lines.len();

    for (i, (word, _)) in lines.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", word);
        for (_, line) in &lines[len - i - 1..len] {
            println!("{}", line);
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use crate::ttdoc::ttdoc;

    #[test]
    fn it_prints_ttdoc() {
        ttdoc()
    }
}

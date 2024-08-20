fn main() {
    let gifts = [
        "A patridge in a pear tree.",
        "Two turtle doves,",
        "Three French Hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Tweleve drummers drumming,"
    ]; //0..11

    for gifts_index in 0..=11 {
        let suffix: &str = match gifts_index + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        };

        for count in (0..=gifts_index).rev() {
            if count == gifts_index {
                println!("On the {}{} day of christmas,\nmy true love gave to me,", gifts_index + 1, suffix);
            }
            println!("{}", gifts[count]);
        }
        println!();
    }
}
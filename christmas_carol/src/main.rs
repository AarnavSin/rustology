fn main() {
    // 12 Days of Christmas Carol

    let lyrics = [
        "A partridge in a pear tree.",
        "Two turtle doves, and",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ]; // 0 to 11

    let numbered_days = [
        "1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th",
    ];

    for count in 0..12 {
        let suffix: &str = match count + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    }
}

fn main() {
    const DAYS: [(&str, &str); 12] = [
        ("first", "partridge in  a pear tree"),
        ("second", "Two turtle doves"),
        ("third", "Three French hens"),
        ("forth", "Four calling birds"),
        ("fifth", "Five gold rings"),
        ("sixth", "Six geese a laying"),
        ("seventh", "Seven swans a swimming"),
        ("eight", "Eight maids a milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming")
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas", DAYS[i].0);
        println!("My true love gave to me");

        for j in (0..i + 1).rev() {
            if i == 0 && j == 0 {
                println!("A {}\n", DAYS[j].1);
            } else if j == 0 {
                println!("And a {}\n", DAYS[j].1);
            } else {
                println!("{}", DAYS[j].1);
            }
        }
    };
}

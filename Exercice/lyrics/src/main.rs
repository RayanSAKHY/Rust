fn main() {
    let number = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let content = [
        "Two turtle doves,\nAnd a partridge in a pear tree.",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let line = "---------------------------------------------------------";
    println!("The 12 Days of Christmas Lyrics\n{line}\n{line}");

    for i in 0..12 {
        println!(
            "On the {} day of Christmas,\nmy true love sent to me",
            number[i]
        );

        if i == 0 {
            print!("A partridge in a pear tree.\n");
        } else {
            for j in (0..i).rev() {
                println!("{}", content[j]);
            }
        }
        println!("");
        if i % 2 == 0 && i != 0 {
            println!("{line}\n");
        }
    }
}

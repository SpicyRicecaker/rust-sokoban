// Generate nth fibonacci number based off of user input
pub fn sing() {
    let chorus: &str = "On the 12th day of Christmas my true love sent to me";
    let verse: [&str; 12] = [
        "12 drummers drumming",
        "11 pipers piping",
        "10 lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];

    for day in 0..12 {
        // 11- i;
        println!("{}", chorus);
        // Then print the rest of the verbs
        for verse_num in 12 - day..12 {
            println!("{}", verse[verse_num]);
        }
        println!();
    }
}

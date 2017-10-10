fn main() {
    let gifts = [
        "a partridge in a pair tree.",
        "two turtle doves,",
        "three french hens,",
        "four calling birds,",
        "five golden rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,"
    ];

    let ordinals = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    for i in 0..12 {
            println!("On the {} day of Christmas, my true love gave to me",
                     ordinals[i]);
        for g in (0..i+1).rev() {
            println!("{}", gifts[g]);
        }
    }

}

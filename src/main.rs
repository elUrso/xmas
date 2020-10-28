extern crate inflector;
use inflector::Inflector;

fn main() {
    println!("[The Twelve Days of Christmas]");

    let day = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    for i in 0..day.len() {
        println!();
        println!("On the {} day of Christmas my true love sent to me", day[i]);
        if i == 0 {
            println!("{}.", gifts[0].to_sentence_case());
        } else {
            for j in (0..i).rev() {
                println!("{},", gifts[j + 1].to_sentence_case());
            }
            println!("And {}.", gifts[0]);
        }
    }
}

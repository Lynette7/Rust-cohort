// In your program, you have been given a string "the lazy fox jumped the bridge". Count the number of occurrences for each character in the string, and the no. of occurrences
// for each word

use std::collections::HashMap;
fn main() {
    character_number();
    word_number();
}

fn character_number() {
    let sentence = "the lazy fox jumped the bridge";
    let vector :Vec<char> = sentence.chars().collect();

    let mut total = HashMap::new();

    for i in vector{
        let count = total.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}",total);
}

fn word_number() {
    let sentence = "the lazy fox jumped the bridge lazy";
    // let vector :Vec<char> = sentence.chars().collect();

    let mut total = HashMap::new();

    for i in sentence.split_whitespace(){
        let count = total.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}",total);
}

//Create a function that takes in two generic parameter types then does a print line and then call it. Make the function return the second parameter
use std::fmt::Debug;

fn main() {
    let millan = 1;
    let auth = "Jenny";
    library(millan, auth);
    println!("The author is {}", library(millan, auth))
}

fn library<T: Debug, X: Debug>(book: T, author: X) -> X {
    println!("This library contains book {:?} which was authored by {:?}", book, author);
    author
}

use std::fmt::{Debug, Display};

fn main() {
    let numbers = [12, 30, 4, 59, 23];
    let result = largest_integer(&numbers);
    println!("Largest number is: {}", result);

    let characters = ['f', 't', 'a', 'z', 'j'];
    let cresult = largest_character(&characters);
    println!("Largest character is: {}", cresult);

    let one = largest(&numbers);
    println!("Generics largest number is: {}", one);

    let two = largest(&characters);
    println!("Generic largest character is: {}", two);

    let number: i32 = 33;
    let text_str: &str = "Hello world!";

    print_debug(number);
    print_debug(text_str);

    print_display(number);
    print_display(text_str);
}

fn largest_integer (list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &i in list.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn largest_character (list: &[char]) -> char {
    let mut largest = list[0];
    for &i in list.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}

// How do we write generic functions that work for all concrete types?
fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list.iter() {
        if i > largest {
            largest = i;

        }
    }
    largest
}

// Display trait is primarily used to display data
// Debug trait uses {:?} instead of {} when printing
fn print_debug<T: Debug> (item: T) {
    println!("{:?}", item);
}

fn print_display<T: Display> (item: T) {
    println!("{}", item)
}

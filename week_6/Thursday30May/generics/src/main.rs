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

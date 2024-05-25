//create a vector and print out the 7th 8th and 9th element

fn main() {
    let vectors = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut i = 0; //u

        //using for loop
    // for i in 0..vectors.len() {
    //     if i >= 6 && i < 9 {
    //         println!("{}th element is {}", i+1, vectors[i]);
    //     }

    // }

        //using while loop
    while i < vectors.len() {
        if i >= 6 && i < 9 {
            println!("{}th element is {}", i+1, vectors[i]);
        }
        i += 1;
    }

}

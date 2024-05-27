// Write a Rust function that takes a vector of tuples (i32, i32) and returns the sum of all the first elements if the second elements are all even, the sum of all the second 
// elements if the first el%2ements are all odd, and 0 otherwise.

fn main() {
    let tup = vec![(3, 2), (17, 2)];
    prinln!
    tuple_vectors(tup);
}

fn tuple_vectors(tup_vec :Vec<(i32, i32)>) -> i32 {
    // if tuple1.1 % 2 == 0 && tuple2.1 % 2 == 0 {
    //     let sum1 = tuple1.0 + tuple2.0;
    //     sum1
    // } else if tuple1.0 % 2 == 1 && tuple1.0 % 2 == 1 {
    //     let sum2 = tuple1.1 + tuple2.1;
    //     sum2
    // } else {
    //     return 0;
    // }

    let all_first_element_odd = true;
    let all_second_element_even = true;

    for (item1, item2) in tup_vec {
        if  item2 % 2 != 0 {
            all_second_element_even=false
        }
        if item1 % 2 == 0 {
            all_first_element_odd= false
        }
    }

    if all_second_element_even {
        //
    }
    if all_first_element_odd{

    }
    0
}

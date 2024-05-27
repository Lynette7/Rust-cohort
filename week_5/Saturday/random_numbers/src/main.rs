// use rand crate to generate 10 numbers and prints them in descending order
use rand::Rng;

fn main() {
    let mut randoms = rand::thread_rng();
    let mut num_vec :Vec<i32> = Vec::new();
    let n = 10;
    let mut i = 0;
    while i < n {
        let number :i32 = randoms.gen_range(0..100);
        num_vec.push(number);
        i += 1;
    }

    println!("{:?}", num_vec);
    num_vec.sort_by(|a, b| b.cmp(a));
    println!("{:?}", num_vec);
    // num_vec.sort();
}

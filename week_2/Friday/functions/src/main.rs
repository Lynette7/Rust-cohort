fn main() {
    // sum - takes in two parameters
    let age = {
        123 * 44
    };
    println!("hey, age {}", age);

    sum(1, 54);
    sum1(75, 33);
    println!("{}", sum2(30, 64));
    // sum2(30, 64);
}

fn sum(num1 :i32, num2 :i32) {
    let num3 = num1 + num2;
    println!("Sum of {} and {} is {}", num1, num2, num3);
}

fn sum1(num1 :i32, num2 :i32) -> i32 {
    let num3 = num1 + num2;
    println!("Sum of {} and {} is {}", num1, num2, num3);
    return num3;
}

fn sum2(num1 :i32, num2 :i32) -> i32 {
    let num3 = num1 + num2;
    num3
}

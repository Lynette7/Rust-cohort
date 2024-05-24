//What are the differences between a const and let other than assignment?

fn main() {
    println!("DIFFERENCES BETWEEN CONST AND LET IN RUST!");
    println!("1. let is used to assign values to variables, const is used to assign values to constants.");
    println!("2. Variables are immutable unless 'mut' keyword is used, constants are forever immutable.");
    println!("3. Variables' data types can or cannot be declared, constants' data types MUST be declared.");
    println!("4. Constants have to be set to a constant expression and not anything that could only be determined at runtime, unlike variables.");
    println!("5. Constants are always fixed in size, unlike variables.");
    println!("6. The naming convention for variables is snake_case, naming convention for constants is SCREAMING_SNAKE_CASE.");
    println!("7. Constants can be declared outside of the main function.");
}

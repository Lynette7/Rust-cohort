fn main() {
    // str - a constant
    // String - array of characters, can push to it

    const MY_AGE :&str = "twenny two";
    let text :String = String::from("Hello there ....");

    let text1 = String::from("Hellooo, hiiii!!!! Can you say mama??? Ma-ma!! Good job!!!");

    //convert one dtata type to another
    let mut text2 = "Welcome to programming".to_string();
    text2.push_str(" Rust!!");

    println!("{MY_AGE}");
    println!("{text}");
    println!("{text2}");
    println!("{text1}");
}

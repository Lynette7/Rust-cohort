// ENUMS
// enums are user-specified types
enum AccountTypes{
    ADMIN(String),
    SUDO(i32),
    NORMAL(f32)
}

enum Cultures<Y>{
    ONE(Y),
    TWO(Y)
}

fn main() {
    let _accountone = AccountTypes::NORMAL(3.6);
    let one = Cultures::ONE(56);
    let once = Cultures::ONE("Hello World!".to_string());
    let two = Cultures::TWO(77);

    match one {
        Cultures::ONE(value) => println!("The value is {}", value),
        Cultures::TWO(x) => println!("Culture is two!")
    }

    match once {
        Cultures::ONE(value) => println!("-> The value is {}", value),
        Cultures::TWO(x) => println!("-> Culture is two!")
    }

    match two {
        Cultures::ONE(value) => println!(" The value is not {}", value),
        Cultures::TWO(x) => println!("Culture is not two!")
    }
}

// Not all types have to be generics eg z in the struct below
struct Point<T> {
    x: T,
    y: T
    // z: i32
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self{
        Point{x, y}
    }

    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

fn main() {
    // let d = Point{x: 1, y: 1};
    // let p = Point{x: 1.4, y: 2.5};

    let my_new_one = Point::new(3.5, 4.7);
    println!("X is at position {}", my_new_one.get_x());
    println!("Y is at position {}", my_new_one.get_y());

    let direction = Compass::new(8.8, "south west");
    println!("N is at position {}", direction.get_n());
    println!("S is at position {}", direction.get_s());

    // If you want to swap the values in the different attributes
    let mut a = 1;
    let mut b = 2;
    swap(&mut a, &mut b);
    println!("a = {} and b = {}", a, b);

    // if we want to swap values of similar attributes of different objects
    let mut car1 = Vehicle::new("KCY", "KDD".to_string());
    let car2 = Vehicle::new("KAZ", "GK01".to_string());
    car1.swapcar(car2);
    assert_eq!(car1.customer_one, "KAZ");
    println!("Customer One car1 {:?}", car1);
}

// How to implement struct generics so that you can have different types for different attributes
struct Compass<T, D> {
    n: T,
    s: D
}

impl<T, D> Compass<T, D> {
    fn new(n: T, s: D) -> Self{
        Compass{n, s}
    }

    fn get_n(&self) ->&T {
        &self.n
    }

    fn get_s(&self) ->&D {
        &self.s
    }
}

// If you want to swap the values in the different attributes
fn swap<T> (x: &mut T, y: &mut T) {
    std::mem::swap(x, y);
}

// if we want to swap values of similar attributes of different objects
#[derive(Debug)]
struct Vehicle<T, X>{
    customer_one: T,
    customer_two: X
}

impl<T,X> Vehicle<T, X>{
    fn new(customer_one: T, customer_two: X) -> Self {
        Vehicle{customer_one, customer_two}
    }

    fn swapcar(& mut self, other_client: Vehicle<T, X>) {
        self.customer_one = other_client.customer_one;
    } 
}

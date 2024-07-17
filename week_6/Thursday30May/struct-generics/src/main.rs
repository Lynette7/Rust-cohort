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

    fn get_x(&self) ->&T {
        &self.x
    }

    fn get_y(&self) ->&T {
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
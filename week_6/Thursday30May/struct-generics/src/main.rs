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
}

// create a traffic light solution which can be in 1 of 3 states i.e red, orange or green. Use match to print: green - go, orange - get ready to go, red - don't go

fn main() {
    let morning = Status::Red;
    let noon = Status::Orange;
    let night = Status::Green;

    traffic_lights(morning);
    traffic_lights(noon);
    traffic_lights(night);
}

enum Status {
    Red,
    Orange,
    Green,
}

fn traffic_lights(state: Status) {
    match state {
        Status::Red => println!("Don't go!!"),
        Status::Orange => println!("Get ready to go!!"),
        Status::Green => println!("Go!!"),
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    use TrafficLight::{Red, Yellow};

    let a = Red;
    let b = Yellow;
    let c = TrafficLight::Green;
}

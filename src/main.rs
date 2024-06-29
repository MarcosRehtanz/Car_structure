#[derive(PartialEq, Debug)]
struct Car {
    color: Color,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

#[derive(PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles == 0 {
        return (Age::New, miles);
    } else {
        return (Age::Used, miles);
    }
}

fn car_factory(color: Color, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        // color: car_color(color),
        color,
        motor,
        roof,
        age: car_quality(miles),
    }
}

fn main() {
    let quality1 = car_quality(0);
    let quality2 = car_quality(500);
    println!("quality1: {:#?}", quality1);
    println!("quality2: {:#?}", quality2);

    let car1 = car_factory(Color::Red, Transmission::Automatic, true, 0);
    let car2 = car_factory(Color::Green, Transmission::Manual, true, 5000);
    let car3 = car_factory(Color::Blue, Transmission::SemiAuto, false, 50);
    println!("car1: {:#?}", car1);
    println!("car2: {:#?}", car2);
    println!("car3: {:#?}", car3);
}

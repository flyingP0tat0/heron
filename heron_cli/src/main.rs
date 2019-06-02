mod input;

use heron::heron;

fn main() {
    println!("square:");
    let square = input::read_line()
        .parse::<f32>()
        .expect("Error parsing input!");
    println!();

    println!("absolute precision:");
    let precision = input::read_line()
        .parse::<f32>()
        .expect("Error parsing input!");
    println!();

    println!("square root: {}", heron(square, precision));
}

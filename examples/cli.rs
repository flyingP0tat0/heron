use heron::heron;
use std::io;

pub fn read_line() -> String {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
        }
        Err(error) => panic!("{}", error)
    }

    input
}

fn main() {
    println!("square:");
    let square = read_line().parse::<f32>().expect("Error parsing input!");
    println!();

    println!("absolute precision:");
    let precision = read_line().parse::<f32>().expect("Error parsing input!");
    println!();

    println!("square root: {}", heron(square, precision));
}

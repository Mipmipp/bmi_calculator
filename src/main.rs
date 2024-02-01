mod errors;
mod messages;
mod utils;
use std::io;

fn main() {
    let mut weight = String::new();
    let mut height = String::new();

    println!("{}", messages::ENTER_WEIGHT);
    io::stdin()
        .read_line(&mut weight)
        .expect(errors::ERROR_READ_WEIGHT);

    println!("{}", messages::ENTER_HEIGHT);
    io::stdin()
        .read_line(&mut height)
        .expect(errors::ERROR_READ_HEIGHT);

   

    let weight_parsed: f64 = utils::parse_input(weight).unwrap()
    let height_parsed: f64 = utils::parse_input(height).unwrap()


    println!("data, {}, {}!", weight_parsed, height_parsed);
}

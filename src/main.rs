pub mod errors;
pub mod messages;
pub mod utils;
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

   

     let weight_parsed = match utils::parse_input(weight) {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing weight: {}", e);
            return;
        }
    };

    let height_parsed = match utils::parse_input(height) {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing height: {}", e);
            return;
        }
    };

    let bmi_value = utils::calculate_bmi(weight_parsed, height_parsed);
    let bmi_formatted = utils::format_bmi(bmi_value);


    println!("{}{}", messages::BMI_RESPONSE ,bmi_formatted);
}

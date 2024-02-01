use crate::errors;

pub fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

pub fn parse_input(input: String) -> Result<f64, String> {
    if input.trim().is_empty() {
        return Err(errors::ERROR_EMPTY_INPUT.to_string());
    }

    let filtered_input: String = input.replace(',', ".").trim().to_string();

    if filtered_input.contains('.') {
         match filtered_input.parse::<f64>() {
            Ok(number) => Ok(meters_to_centimeters(number)),
            Err(_) => Err(errors::ERROR_VALID_NUMBER.to_string()),
        }
    } else {
         match filtered_input.parse::<f64>() {
            Ok(number) => Ok(number),
            Err(_) => Err(errors::ERROR_VALID_NUMBER.to_string()),
        }
    }
}

fn meters_to_centimeters(meters: f64) -> f64 {
    meters * 100.0
}
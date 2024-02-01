use crate::errors;



pub fn parse_input(input: String) -> Result<f64, String> {
    if input.trim().is_empty() {
        return Err(errors::ERROR_EMPTY_INPUT.to_string());
    }

     let filtered_input: String = input.replace('.', "");

     match filtered_input.parse::<f64>() {
        Ok(number) => Ok(number),
        Err(_) => Err(errors::ERROR_VALID_NUMBER.to_string()),
    }
}
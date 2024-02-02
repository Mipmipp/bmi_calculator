pub mod errors;
pub mod messages;
pub mod utils;
use std::io::{self, BufRead, Write};
use std::error::Error;

fn run<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> Result<(), Box<dyn Error>> {
    let mut weight = String::new();
    let mut height = String::new();

    write!(output, "{}", messages::ENTER_WEIGHT)?;
    output.flush()?;

    input.read_line(&mut weight)?;

    write!(output, "{}", messages::ENTER_HEIGHT)?;
    output.flush()?; 

    input.read_line(&mut height)?;

    let weight_parsed = utils::parse_input((&weight.trim()).to_string())?;
    let height_parsed = utils::parse_height_input((&height.trim()).to_string())?;
 
    let bmi_value = utils::calculate_bmi(weight_parsed, height_parsed);
    let bmi_formatted = utils::format_number_with_2_decimals(bmi_value);

    writeln!(output, "{}{}", messages::BMI_RESPONSE, bmi_formatted)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run(&mut io::stdin().lock(), &mut io::stdout())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, BufReader};

    #[test]
    fn test_with_number_with_valid_input() -> Result<(), Box<dyn Error>> {
        const BMI_FORMATTED_RESULT: &str = "16.98";
        let expect_output: String = format!("{}{}", messages::BMI_RESPONSE, BMI_FORMATTED_RESULT);

        let input_data = "55\n1.80\n";
        let mut input = BufReader::new(Cursor::new(input_data.as_bytes()));
        let mut output = Vec::new();

        let result = run(&mut input, &mut output);

        let output_str = String::from_utf8(output)?;
   
        assert!(result.is_ok());
        assert!(output_str.contains(&expect_output));

        Ok(())
    }

    #[test]
    fn test_with_number_with_invalid_input() -> Result<(), Box<dyn Error>> {
        let input_data = "\n1.80.0\n";
        let mut input = BufReader::new(Cursor::new(input_data.as_bytes()));
        let mut output = Vec::new();

        let result = run(&mut input, &mut output);

        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.to_string(), errors::ERROR_EMPTY_INPUT);

        Ok(())
    }
}

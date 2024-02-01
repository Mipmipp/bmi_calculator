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
    let height_parsed = utils::parse_input((&height.trim()).to_string())?;

    let bmi_value = utils::calculate_bmi(weight_parsed, height_parsed);
    let bmi_formatted = utils::format_bmi(bmi_value);

    writeln!(output, "{}{}", messages::BMI_RESPONSE, bmi_formatted)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run(&mut io::stdin().lock(), &mut io::stdout())
}

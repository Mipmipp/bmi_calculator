use crate::errors;

pub fn calculate_bmi(weight: f64, height_cm: f64) -> f64 {
    let height_m = centimeters_to_meters(height_cm);
    weight / (height_m * height_m)
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

pub fn format_bmi(bmi: f64) -> String {
    format!("{:.2}", bmi)
}

fn meters_to_centimeters(meters: f64) -> f64 {
    meters * 100.0
}

fn centimeters_to_meters(centimeters: f64) -> f64 {
    centimeters / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEIGHT_M: f64 = 1.65;
    const HEIGHT_CM: f64 = 165.0;
    
    #[test]
    fn test_calculate_bmi() {
        const WEIGHT: f64 = 75.0;
        let expected = WEIGHT / (HEIGHT_M * HEIGHT_M);
        assert_eq!(calculate_bmi(WEIGHT, HEIGHT_CM), expected);
    }

    #[test]
    fn test_parse_input_empty() {
        let input = String::new();
        let result = parse_input(input);
        assert!(matches!(result, Err(ref e) if e == errors::ERROR_EMPTY_INPUT));
    }
    
    #[test]
    fn test_parse_input_with_invalid_number() {
        let input = "1.345.435".to_string();
        let result = parse_input(input);
        assert!(matches!(result, Err(ref e) if e == errors::ERROR_VALID_NUMBER));
    }

   #[test]
    fn test_parse_input_with_number_with_decimal() {
        let input = "1.345".to_string();
        let result = parse_input(input);
        assert!(matches!(result, Ok(number) if (number - 134.5).abs() < f64::EPSILON));
    }

    #[test]
    fn test_parse_input_with_number_without_decimal() {
        let input = "1".to_string();
        let result = parse_input(input);
        assert!(matches!(result, Ok(number) if (number - 1.0).abs() < f64::EPSILON));
    }

    #[test]
    fn test_format_bmi() {
        const BMI_UNFORMATTED: f64 = 23.0654232;
        let expected: String = format!("{:.2}", BMI_UNFORMATTED);
        assert_eq!(format_bmi(BMI_UNFORMATTED), expected);
    }

    #[test]
    fn test_meters_to_centimeters() {
        let expected = HEIGHT_M * 100.0;
        assert_eq!(meters_to_centimeters(HEIGHT_M), expected);
    }

    #[test]
    fn test_centimeters_to_meters() {
        let expected = HEIGHT_CM / 100.0;
        assert_eq!(centimeters_to_meters(HEIGHT_CM), expected);
    }
}

use crate::errors;

pub enum BmiClassification {
    Underweight,
    NormalWeight,
    Overweight,
    Obese,
}

const UNDERWEIGHT_THRESHOLD: f64 = 18.5;
const NORMAL_WEIGHT_THRESHOLD: f64 = 25.0;
const OVERWEIGHT_THRESHOLD: f64 = 30.0;

pub fn determine_bmi_classification(bmi_value: f64) -> BmiClassification {
    if bmi_value < UNDERWEIGHT_THRESHOLD {
        BmiClassification::Underweight
    } else if bmi_value < NORMAL_WEIGHT_THRESHOLD {
        BmiClassification::NormalWeight
    } else if bmi_value < OVERWEIGHT_THRESHOLD {
        BmiClassification::Overweight
    } else {
        BmiClassification::Obese
    }
}

pub fn calculate_bmi(weight: f64, height_cm: f64) -> f64 {
    let height_m = centimeters_to_meters(height_cm);
    weight / (height_m * height_m)
}

pub fn parse_height_input(input: String) -> Result<f64, String> {
    let validated_input = validate_input(input)?;   

    let filtered_input: String = validated_input.replace(',', ".").trim().to_string();

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

pub fn parse_weight_input(input: String) -> Result<f64, String> {
    let validated_input = validate_input(input)?;   

  
    match validated_input.parse::<f64>() {
        Ok(number) => Ok(format_number_without_decimals(number)),
        Err(_) => Err(errors::ERROR_VALID_NUMBER.to_string()),  
    }
    
}

pub fn validate_input(input: String) -> Result<String, String> {
    if input.trim().is_empty() {
        return Err(errors::ERROR_EMPTY_INPUT.to_string());
    }

    if input.trim() == "0" {
        return Err(errors::ERROR_NUMBER_IS_ZERO.to_string());
    }

    Ok(input)
}

pub fn format_number_with_2_decimals(number: f64) -> String {
    format!("{:.2}", number)
}

pub fn format_number_without_decimals(number: f64) -> f64 {
    number.floor()
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
    fn test_underweight() {
        let bmi_value = 17.5;
        matches!( determine_bmi_classification(bmi_value), BmiClassification::Underweight);
    }
 
    #[test]
    fn test_normal_weight() {
        let bmi_value = 25.0;
        matches!( determine_bmi_classification(bmi_value), BmiClassification::NormalWeight);
    }

    #[test]
    fn test_overweight() {
        let bmi_value = 30.0;
        matches!( determine_bmi_classification(bmi_value), BmiClassification::Overweight);
    }

    #[test]
    fn test_obese() {
        let bmi_value = 40.0;
        matches!( determine_bmi_classification(bmi_value), BmiClassification::Obese);
    }

    #[test]
    fn test_calculate_bmi() {
        const WEIGHT: f64 = 75.0;
        let expected = WEIGHT / (HEIGHT_M * HEIGHT_M);
        assert_eq!(calculate_bmi(WEIGHT, HEIGHT_CM), expected);
    }

    #[test]
    fn test_validate_input_valid() {
        let input = "1".to_string();
        let result = validate_input(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_input_empty() {
        let input = String::new();
        let result = validate_input(input);
        assert!(matches!(result, Err(ref e) if e == errors::ERROR_EMPTY_INPUT));
    }
    
    #[test]
    fn test_validate_input_with_zero() {
        let input = "0".to_string();
        let result = validate_input(input);
        assert!(matches!(result, Err(ref e) if e == errors::ERROR_NUMBER_IS_ZERO));
    }

    #[test]
    fn test_parse_height_input_valid_number() {
        let input = "1.65".to_string();
        assert_eq!(parse_height_input(input).unwrap(), HEIGHT_CM);
    }
 
    #[test]
    fn test_parse_height_input_with_invalid_number() {
        let input = "1.345.435".to_string();
        let result = parse_height_input(input);
        assert!(matches!(result, Err(ref e) if e == errors::ERROR_VALID_NUMBER));
    }

   #[test]
    fn test_parse_height_input_number_with_decimal() {
        let input = "1.345".to_string();
        let result = parse_height_input(input);
        assert!(matches!(result, Ok(number) if (number - 134.5).abs() < f64::EPSILON));
    }

    #[test]
    fn test_parse_height_input_with_number_without_decimal() {
        let input = "1".to_string();
        let result = parse_height_input(input);
        assert!(matches!(result, Ok(number) if (number - 1.0).abs() < f64::EPSILON));
    }

    #[test] 
    fn test_parse_weight_input_with_number_with_decimal() {
        let input = "75.45".to_string();
        assert_eq!(parse_weight_input(input).unwrap(), 75.0);
    }

    #[test]
    fn test_format_bmi() {
        const BMI_UNFORMATTED: f64 = 23.0654232;
        let expected: String = format!("{:.2}", BMI_UNFORMATTED);
        assert_eq!(format_number_with_2_decimals(BMI_UNFORMATTED), expected);
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

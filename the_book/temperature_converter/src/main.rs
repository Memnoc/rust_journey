// NOTE:
// Here's the formulas to convert both temperatures
// temp = (temp * 9 / 5) + 32; for Celsius
// temp = ((temp - 32) * 5) / 9; for Farenheit
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Temperature Converter ==");
    println!("Type 'Q' at any prompt to quit.");

    loop {
        println!("== Select the unit: (C) or (F) ==");
        let mut unit = String::new();

        io::stdin().read_line(&mut unit)?;

        if unit.trim().eq_ignore_ascii_case("q") {
            println!("Exiting program.");
            break;
        }

        let unit = unit.trim().to_uppercase();
        let prompt = format!("Please enter the temperature in {}: ", unit);

        match unit.as_str() {
            "C" | "F" => match read_temperature(&prompt) {
                Ok(temp) => match convert_temperature(temp, &unit) {
                    Ok(converted) => println!(
                        "The temperature in {} is {}",
                        if unit == "C" { "F" } else { "C" },
                        converted
                    ),
                    Err(e) => println!("{:#?}", e),
                },
                Err(_) => println!("Failed to read temperature."),
            },
            _ => println!("Enter a correct unit (C or F)"),
        }
    }
    Ok(())
}

//NOTE: custom errors
#[derive(Debug)]
enum TemperatureError {
    InvalidUnit,
    OutOfRange,
    ReadError(String),
}
fn convert_temperature(temp: f32, unit: &str) -> Result<f32, TemperatureError> {
    if temp < -273.15 {
        return Err(TemperatureError::OutOfRange);
    }
    match unit {
        "C" => Ok((temp * 9.0 / 5.0) + 32.0),
        "F" => Ok((temp - 32.0) * 5.0 / 9.0),
        _ => Err(TemperatureError::InvalidUnit),
    }
}

fn read_temperature(prompt: &str) -> Result<f32, TemperatureError> {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut temperature = String::new();

    if io::stdin().read_line(&mut temperature).is_err() {
        return Err(TemperatureError::ReadError(
            "Failed to read temperature.".to_string(),
        ));
    }

    match temperature.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(TemperatureError::ReadError(
            "Invalid temperature format.".to_string(),
        )),
    }
}

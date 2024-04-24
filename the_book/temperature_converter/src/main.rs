// NOTE:
// Here's the formulas to convert both temperatures
// temp = (temp * 9 / 5) + 32; for Celsius
// temp = ((temp - 32) * 5) / 9; for Farenheit

use std::io;
use std::io::Write;

fn convert_temperature(temp: f32, unit: &str) -> Result<f32, String> {
    match unit {
        "C" => Ok((temp * 9.0 / 5.0) + 32.0),
        "F" => Ok((temp - 32.0) * 5.0 / 9.0),
        _ => Err("Please enter the correct unit".to_string()),
    }
}

fn read_temperature(prompt: &str) -> Result<f32, io::Error> {
    loop {
        print!("{prompt}");
        io::stdout().flush()?;
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)?;

        match temperature.trim().parse() {
            Ok(num) => return Ok(num),
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

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
                    Err(e) => println!("{}", e),
                },
                Err(_) => println!("Failed to read temperature."),
            },
            _ => println!("Enter a correct unit (C or F)"),
        }
    }
    Ok(())
}

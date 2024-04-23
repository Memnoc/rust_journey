use std::io;
fn main() {
    println!("== Temperature Converter ==");

    loop {
        println!("== Select the unit: (C) or (F) ==");
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let unit = unit.trim().to_uppercase(); // allows for lower case input

        match unit.as_str() {
            "C" => println!("This is C"),
            "F" => println!("This is F"),
            _ => println!("Enter a correct unit"),
        }
    }
}

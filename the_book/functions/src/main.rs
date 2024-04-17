fn main() {
    println!("== Functions ==");

    // NOTE: calling another function
    another_function();

    // NOTE: calling a function with parameters
    let x = 34;
    second_function(x);

    // NOTE: calling a function with multiple parameters
    let y = 5;
    let unit = 'h';
    print_labeled_measurement(y, unit);

    // NOTE: calling function with return type
    let name = "Jim";
    third_function(name);
}

// NOTE: declaring another_function
fn another_function() {
    println!("Another function.");
}

// NOTE:: parameters must specify the type
fn second_function(x: i32) {
    println!("The value of x is {x}");
}

// NOTE: function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// NOTE: if there is a return, that must be specified along with the type
fn third_function(name: &str) -> &str {
    println!("Hello {name}");
    name
}

fn main() {
    println!("== Control Flow ==");
    let number = 6;

    // NOTE: this is the classic if
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // NOTE: this is divergent from the classical truthy model
    if number != 0 {
        println!("Number is not 0");
    }

    // NOTE: multiple if/else are also possible
    // although really hughly - use a match instead
    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number}umber is divisible by 2");
    } else {
        println!("{number}umber is not divisible by 4, 3 or 2");
    }

    // INFO: If is an expression and as such we can use it
    // on the right side of a variable assignment
    // very cool!

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // WARN: one very important thing:
    // values of the arms MUST BE of the same type!

    // FIX: this cannot be
    // let number = if condition {5} else {"six"};
}

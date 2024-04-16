fn main() {
    println!("== Variables ==");

    //NOTE: without the word mut, re-assignment is not possible
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // NOTE: with constants, you need all caps name and declare the type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

    // NOTE: shadowing is like a temp mutation
    // let's you create a new variable using the same name
    // but when the variable scope drops, the variable returns to the previous state
    // this means you can mutate immutable variables inside a given scope
    let y = 6;
    let y = y + 6;
    println!("The value of y OUTSIDE BEFORE is {y}");

    {
        println!("The value of y INSIDE BEFORE is {y}");
        let y = y * 2;
        println!("The value of y INSIDE is {y}");
    }
    println!("The value of y OUTSIDE is {y}");

    // NOTE: because shadowing effectively creates a new variable, we can change the type
    let spaces = "   "; // string
    println!("spaces BEFORE is {spaces}");
    let spaces = spaces.len(); // number
    println!("spaces AFTER is {spaces}");
}

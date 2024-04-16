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
}

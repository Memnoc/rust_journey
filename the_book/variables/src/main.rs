fn main() {
    println!("== Variables ==");

    //NOTE: without the word mut, re-assignment is not possible
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

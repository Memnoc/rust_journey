fn main() {
    println!("== Varialbes ==");
    //NOTE: without the word mut the re-assignment causes an error
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

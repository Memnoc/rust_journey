// INFO: just a way to test the concepts fromt the book
fn main() {
    println!("=== Playground ===");

    let mut x = 10; // popped off of stack
    let y = x; // y now owns it
    println!("{} {}", x, y); // this is fine

    {
        x = 10;
        println!("Another x: {}", x);
    }
}

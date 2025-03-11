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
    {
        let s = "hello";
        //s.push_str(", world"); // &str cannot be mutated

        // do stuff with s
    } // s is now out of scope
    {
        // NOTE: why can String be mutated but literal &str cannot?
        // String is known at compile time, it's hardcoded and its size is known
        // &str is dynamic, so the compiler does not know the size, hence it stays in the heap
        let mut s = String::from("hello"); // hardcoded and fixed size

        s.push_str(", world"); // push_str() appends a literal to a String

        println!("{s}"); // this will print `hello, world`
    }
    {
        let s1 = String::from("hello");
        let s2 = s1; // s1 dropped out of scope now

        println!("{s1}, world"); // cannot be used here
                                 // the data has been "moved" from s1 to s2
    }
    {
        // FIX: same example but fixed
        let s1 = String::from("Hello");
        let s2 = s1.clone(); // now we have a clone

        println!("s1 = {s1}, s2 = {s2}");
    }
    {
        // NOTE: same example but with a different data type
        // this works because this type is on the stack
        // as it has known value in real at compile time
        let x = 5;
        let y = x;

        println!("x = {x}, y = {y}");
    }
}

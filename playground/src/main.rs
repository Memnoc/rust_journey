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

        // println!("{s1}, world"); // cannot be used here
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

    // HEADER: ownership with functions
    {
        let s = String::from("hello"); // in scope

        takes_ownership(s); // function takes ownership of s
                            // s value is moved into the function
                            // s (alone) is no longer valid

        // println!("{s}"); // error
        let x = 5; // x into scope

        makes_copy(x); // x moved into the function
                       // can still use x because it's Copy (trait) and stack
        println!("{x}");
    }

    fn takes_ownership(some_string: String) {
        // some_string in scope
        println!("some_string");
    } // some_string out of scope and drop() is called
      // memory is freed

    fn makes_copy(some_integer: i32) {
        // some_integer in scope
        println!("some_integer");
    } // some_integepr goes out of scope but that's it (popped out of the stack, no drop())
    {
        // INFO: return values can transfer ownership
        let s1 = gives_ownership(); // gives_ownership moves its return to s1

        let s2 = String::from("s2"); // s2 into scope

        let s3 = takes_and_gives_back(s2); // s2 moves into the function
                                           // s3 gives back its ownership to s3 by returning
        println!("s3 taking ownership of = {s3}");
        fn gives_ownership() -> String {
            let some_string = String::from("yours");

            some_string // here the function returns, giving ownershipt to s1
        }

        // NOTE: takes a string but also gives it back
        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }
    }
    {
        // INFO: let's see how borrowing helps the situation
        let s1 = String::from("hello");

        let len = calculate_len(&s1); // passing a reference of s1 to the function

        println!("The length of '{s1} is {len}");

        // NOTE: in the example is &String which creates a new object
        // But should be &str which is a slice
        //https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
        fn calculate_len(s: &String) -> usize {
            // function borrows s1, does stuff with it, and
            // then returns the value - no ownership taken
            s.len()
        }
    }
}

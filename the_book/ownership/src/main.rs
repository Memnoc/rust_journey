// NOTE:
// 1. Each value has an owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

fn main() {
    println!("== Ownership ==");

    // NOTE:
    // https://blog.logrocket.com/understanding-rust-string-str/#:~:text=We%20can%20use%20both%20String,%2C%20it%20won't%20compile.
    // String
    //  - owned type that needs to be allocated
    // &String
    //  - is a reference to a String
    //  - not owned and size is unknown because it's only a pointer to an actual String
    //  - we can pass this around as long as the original reference is in scope
    // &str
    //  - this is a pointer to a memory space and includes the size
    //  - it is known at compile time
    //  - its memory can be on the heap, stack or even static
    //  - it is not owned, it is only a reference to a string slice
    //  - while &str is in scope, the underline memory does not change, even across threads
    //  - &String can be coerced into &str (but not the other way around)
    //

    {
        let s_literal = "Hello"; // literal, immutable, stack
        let mut s = String::from("hello"); // string, mutable, heap

        s = "goodbye".to_string();
        // s_literal = "goodbye"; // FIX: cannot mutate this
    }

    {
        let s3 = String::from("hello s3"); // this is called a "move"
        let s4 = s3;
        println!("{}", s4);
    }

    {
        let s1 = String::from("hello s1");
        let s2 = s1.clone(); // this copies the heap data as well
        print!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        // Stack-only data copy
        let x = 5;
        let y = x; // this is possible because integers types have known size at compile time
        println!("x = {}, y = {}", x, y);
    }
}

## Ownership

How Rust manages memory.

**Stack and Heaps**
Both are parts of memory available to use at runtime. The problems _ownership_ addresses are: keeping track of what parts of code are stored are using what data on the heap, minimising the amount of duplicate data on the heap, cleaning up unused data on the heap so you don't run out of space.

- _Stack_: **all data stored on the stack must have a fixed size**

  - _last in, first out_ stores values in the order it gets them and removes them in the opposite order. So basically the stack removes values from the top of the stack and so on.
  - Adding data is _pushing onto the stack_ to the stack and removing data is _popping off the stack_.
  - Pushing to the stack is faster than allocating to the heap (allocator does not have to search for a place to store memory, it's always at the top of the stack)
  - Modern processors are faster if they jump around less.
  - When the code calls a function, the values passed into the function (including any pointers to data on the heap) and the function's local variables are pushed onto the stack.
  - When the function is over, the values are popped off the stack.

- _Heap_: **all data stored on the stack must have a fixed size**

  - The heap is less organised. Data can be of any size, and the memory allocator takes care of finding a big enough spot. Once found, it returns a pointer, and because the pointer's size is known at compile time, **that pointer is stored in the stack** To retrieve the actual data, however, you must follow the pointer.
    _allocating on the heap_ is the process of requesting memory from the memory allocator.

**Ownership rules**
To keep in mind:

- Each value in Rust had an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

**The String Type**
It's easier to look at the string type when it comes to explain ownership, as that is data that has to be stored in the heap and it related to more complex data types than integer and booleans.

- `String` can be mutated. Its content it's allocated by the memory allocator so it can change - in other words, it's allocated on the heap.

```rust
let mut s = String::from("hello");
s.push_str(", world");
println!("{}", s);
```

- `string literals` cannot be mutated (immutable), as the content is known and allocated at compile time - in other words, it's stored in the stack.

```rust
let s = "Hello";
```

**Why**

When we do `String::from` we are asking for the memory. In Rust, we do not have the classic garbage collection approach (like in JavaScript and Java) and we do not have the manual way either, like in C.
**In Rust, memory is automatically returned once the variable that owns it goes out of sscope**

```rust
{
  let s = String::from("hello"); // s is valid from this point forward

  // do stuff with s
                                // the scope is now over
}                               // no longer valid, memory returned
```

There is a natural point in which we can return the memory, and Rust makes use of that under the hood calling a `drop` function automatically at the closing curly bracket.

**When a variable that includes data on the heaps goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable**

This practically means that Rust invalidates variables that are out of scope, keeping effectively the memory free. It does so by copying the old data from the stack to a new stack, pointing to the same pointer in the heap. So the heap data in this case **is not copied.**
This seems similar to the concept of _shallow copy_ of other programming language. This also means that **Rust will never automatically create "deep" copies of the data. Any _automatic_ copying is inexpensive in terms of runtime performance.**

**Clone and deep copies**
If we want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`. When you see `clone` you know that some special code is being executed and may be expensive.

**Stack-Only Data: Copy**
As said, some types do not need `clone` because they are stored entirely on the stack. In other words, there is not difference between shallow and deep copy.

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Under the cover, Rust uses a special annotation called the `Copy` the trait, and uses that on traits that are stored exclusively on the stack. Types that use `Copy` do not move, rather, are shallowed copied which makes them still valid after assignment to another variable.

Rust does not let you annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.

Generally speaking, any of the simple scalar values can implement `Copy`, and nothing that requires allocation or is some of resource can implement `Copy`.

Types tha implement `Copy`:

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

**Ownership and Functions**
Passing values to a function remains very similar to the concepts explored earlier, so we have:

```rust
fn main() {
  let s = String::from("hello"); // s in scope

  takes_ownership(s); // s in owned by the function now
  // can no longer use s outside

  let x = 5; // x in scope

  makes_copy(x); // this we can do because it makes a copy
}




fn takes_owenership(some_str: String) { // some_str into scope
  println!("{}", some_str);
} // drop is called
// memory reclaimed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} // goes out of scope, nothing special happens
```

**Return values and scope**
Returning values can also transfer ownership.

```rust
fn main() {
  let s1 = gives_ownership(); // moves its return value to s1

  let s2 = String::from("hello"); // in scope now

  let s3 = takes_and_gives_back(s2); // s2 moves into this function, nothing special
                                     // the function moves its return value to s3
} // s3 goes out of scope here and it's dropped
  // s2 was moved so nothing happens
  // s1 is out of scope and it's dropped

fn gives_ownership() -> String { // moving return value into the function


  let some_string = String::from("yours"); // this comes into scope

  some_string // this is now returned and moves as returning value to the function
}

// Takes a string and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
  // scope
 a_string // a_string is returned and moves out to the calling function
}
```

There is also **a way to let a function use a variable without taking ownership** - this feature is called **references**

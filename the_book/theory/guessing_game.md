# Chapter 1

## Guessing Game chapter

In this chapter there are a few important concepts:

- `let`
- `match`
- `cargo new`
- `use std::io`
  this is to bring in the standard library, also called the _prelude_
- `println!`
  is a macro, very recurring concept
- `let mut guess = String::new();`
  this line was interesting, it creates a new string that is mutable and bound to an empty instance of `String`
- ````rustio::stding
  .read_line(&mut guess)```
  this is how we inteake user input
  `read_line` return a `Result` - this is an `enum`, a type that can be in multiple state and we call these states `viariants`
  ````
- `Results` variants are `Ok` and `Err`
  the `expect` method exposed will either display an `Ok` or crash the program if the result is an `Err`
- Rust will yell at you if you do not handle errors, so in this case you must use the `Result` value returned
- This line `let secret_number = rand::thread_rng().gen_range(1..=100)`
  is pretty interesting. We have the calls to the crate methods with the dot notation
  notice the `::` operator to show association
  we also have this weird `1..=100` syntax which is a range expression

  ```

  ```

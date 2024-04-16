fn main() {
    println!("== Data Types ==");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {guess}");

    println!("Floating-Point Types");
    let x = 2.0; //f64
    let y: f32 = 3.0; // f32
    println!("The value of x is {x} and the value of y is {y}");

    println!("== Numeric Operations ==");
    // addition
    let sum = 5 + 10;
    println!("The value of sum is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is {product}");

    // division
    let quotient = 56.7 / 32.3;
    let truncated = -5 / 3; // result in -1
    println!("The value of quotient is {quotient}");
    println!("The value of truncated is {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is {remainder}");

    println!("== The Boolean Type ==");
    let t = true;
    println!("The value of t is {t}");

    let f: bool = false;
    println!("The value of f is {f}");

    println!("== The Character Type ==");
    let c = 'z';
    println!("The value of z is {c}");

    let z: char = 'Z'; // with explicit type annotation
    println!("The value of z is {z}");

    println!("== Compound Types ==");
    println!("== Tuple Type ==");
    // NOTE: combines a number of values of different types
    // into one compound type

    let tup: (i32, f64, u8) = (500, 6.4, 1); // declaration
    let (x, y, z) = tup; // assignment to single variables
    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of x is {z}");

    // NOTE: you can access the tup values
    // with dot notation as well
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five hundred is {five_hundred}");
    println!("The value of six point four is {six_point_four}");
    println!("The value of one is {one}");

    println!("== The Array Type ==");
    let a = [1, 2, 3, 4, 5];
    // NOTE: printing the values
    for el in a {
        println!("The value of a is {el}");
    }

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // NOTE: accessing single elements
    let first = months[0];
    println!("The value of first is {first}");
    let second = months[1];
    println!("The value of first is {second}");

    for el in months {
        println!("{el}");
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for el in a {
        println!("{el}");
    }
}

fn calculate_fibonacci_iterative(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut fibonacci: Vec<usize> = vec![0, 1];
            for i in 2..=n {
                let next = fibonacci[i - 1] + fibonacci[i - 2];
                fibonacci.push(next);
            }
            fibonacci[n]
        }
    }
}

fn main() {
    println!("****** nth Fibonacci generator ******");
    let t: usize = 10;

    let fibonacci_number = calculate_fibonacci_iterative(t);
    println!(
        "[ITERATIVE]The {}th Fibonacci number is: {}",
        t, fibonacci_number
    );
}

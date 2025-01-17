// Fix the errors
fn main() {
    let n: f64 = 5.0;

    let big_n =
        if n < 10.0 && n > -10.0 {
            println!(", and is a small number, increase ten-fold");

            10.0 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0
        };

    println!("{} -> {}", n, big_n);
} 
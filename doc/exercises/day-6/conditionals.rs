#![allow(dead_code)]

fn main() {
    let n = 5;

    // plain if-else statements
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    // putting conditionals in an expression
    let big_n = {
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n >> 1
        }
    };

    println!("{} -> {}", n, big_n);
}


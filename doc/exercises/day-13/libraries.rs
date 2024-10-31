#![allow(unused)]
#![allow(dead_code)]
#![allow(non_snake_case)]

mod classes {
    pub mod     methods;
}

mod composers;
mod functions;

use functions::double;
use composers::{compose_with_integer};

fn main () -> () {
    let mut alice: class::Person = class::Person::new(1, 1001);

    println!("created alice of type {}", alice);

    alice.update_SSN(2002021);

    println!("alice is now of type {}", alice);

    let halfer = |x| x / 2;

    let same_8 = compose_with_integer(8, halfer, double);

    println!("8 stays the same as {}", same_8);

    return ();
}

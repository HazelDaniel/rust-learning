#![allow(unused)]

use utils::strings::conversion::{reverse_string, upper_string};

fn main() {
    println!("(last word reversed) Hello, {:?}!", reverse_string("World"));
    println!("(last word upper) Hello, {:?}!", upper_string("World"));
}

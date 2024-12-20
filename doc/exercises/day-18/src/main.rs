#![allow(unused)]

use utils::strings::conversion::{
    reverse_string, rotate_string_l, rotate_string_l_by, rotate_string_r, rotate_string_r_by,
    upper_string,
};

fn main() {
    println!("(last word reversed) Hello, {:?}!", reverse_string("World"));
    println!("(last word upper) Hello, {:?}!", upper_string("World"));
    println!(
        "(last word rotated right) Hello, {:?}!",
        rotate_string_r("World")
    );

    println!(
        "(last word rotated right by 3) Hello, {:?}!",
        rotate_string_r_by("World", 3)
    );

    println!(
        "(last word rotated left) Hello, {:?}!",
        rotate_string_l("World")
    );

    println!(
        "(last word rotated left by 3) Hello, {:?}!",
        rotate_string_l_by("World", 3)
    );
}

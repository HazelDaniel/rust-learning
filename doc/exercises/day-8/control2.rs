#![allow(dead_code)]
#![allow(unused)]

fn age() -> i32 {
    90
}

fn some_number() -> Option<u32> {
    Some(15)
}

// #[derive(PartialEq)]
enum Foo {Bar, Baz(i32)}



fn main () {
    match age() {
        0 => {
            println!("you are literaly zero years old lmao");
        },
        n @ 1..=20 if age() > 10 => {
            println!("i know that you are more than 10 years old and your real age is {n}");
        }
        n @ 1..=20 => {
            println!("i know that you are not more than 10 years of age and your real age is {n}");
        },
        _ => {
            println!("i know you are more than 20 years old but i don't care about your age, really.");
        }
    }

    match some_number() {
        None => {
            println!("there's no number to print.");
        },
        Some (n @ 20..=100) => {
            println!("number is between 20 and 100 (inclusive)");
        },
        Some (n @ 15) => {
            println!("you provided the exact number 15");
        },
        Some (n) => {
            println!("you provided a number i know nothing of");
        },
        _ => ()
    }


    let a = Foo::Baz(2);

    if let Foo::Baz(20) = a {
        println!("a is foobaz 20, okay?");
    }

    return ();
}

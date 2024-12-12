#![allow(unused)]
use options::*;

fn main() {
    println!("Hello, Errors!");

    let person = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                number: 2,
                area_code: None,
            }),
        }),
    };
}

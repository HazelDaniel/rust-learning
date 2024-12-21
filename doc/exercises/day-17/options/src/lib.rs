#![allow(unused)]
#![allow(dead_code)]

use std::num::ParseIntError;

pub mod exports;
pub use exports::utils::utils::*;
// pub use Food::*;

pub fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

pub fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

pub fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    match chopped {
        Some(Chopped(food)) => Some(Cooked(food)),
        None => None,
    }
}

pub fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("we are eating: {:?}", food),
        None => println!("the current food is not edible."),
    };
}

pub fn process(food: Option<Food>) -> Option<Cooked> {
    let ret_peeled_if_food = |x| Some(Peeled(x));
    food.and_then(ret_peeled_if_food)
        // food.map(|x| Peeled(x))
        .map(|Peeled(x)| Chopped(x))
        .map(|Chopped(x)| Cooked(x))
}

fn main() -> Result<(), ParseIntError> {
    let string = "120___";
    let num_parsed;

    let job_number = PhoneNumber {
        number: 2,
        area_code: None,
    };

    let person = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                number: 2,
                area_code: None,
            }),
        }),
    };

    num_parsed = string.parse::<i32>().unwrap_or(-1);

    println!(
        "the work from home number is: {:?} ",
        person.work_phone_area_code().unwrap_or(-1)
    );
    println!("the result value is {:?}", num_parsed);

    println!("the processed food is {:?}", process(Some(Food::Apple)));

    return Ok(());
}

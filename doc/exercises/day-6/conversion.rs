#![allow(dead_code)]
#![allow(unused)]

use std::convert::{TryFrom, TryInto};
use std::fmt;

#[derive(fmt::Debug, PartialEq)]
struct Number {
    value: i32,
}

impl fmt::Display for Number {
    fn fmt (&self, stream: &mut fmt::Formatter) -> fmt::Result {
        return write!(stream, "Number<{}>", self.value);
    }
}

#[derive(fmt::Debug, PartialEq)]
struct Ntype {}


impl TryFrom<i32> for Number {

    type Error = Ntype;

    fn try_from (value: i32) -> Result<Self, Self::Error> {
        if (value % 2 == 0) {
            return Ok(Number {value});
        } else {
            return Err(Ntype {});
        }
    }
}

fn main () {
    let num = Number::try_from(32);
    let integer = 20;

    /*     type annotation needed for into/try_into to work      */
    let res_num: Result<Number, Ntype> = integer.try_into();

    assert_eq!(Number::try_from(3_i32), Err(Ntype{}));
    assert_eq!(Number::try_from(integer), Ok(Number {value: 20}));
    let mut res: i32 = "25".parse::<i32>().unwrap(); // parsed using turbo-fish syntax

    println!("result is {:?}", res);
    res = "20".parse().unwrap(); // parsed using alternate syntax
    println!("result is {:?}", res);

    return ();
}

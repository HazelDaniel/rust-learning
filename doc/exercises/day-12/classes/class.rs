#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]

use std::fmt;

pub struct Person {
    pub id:             i32,
    ssn:                i32,
}

impl Person {
    pub fn new (id: i32, ssn: i32) -> Self {
        Person {id, ssn}
    }

    pub fn update_SSN (&mut self, ssn: i32) -> () {
        self.ssn = ssn;
    }
}

impl fmt::Display for Person {
    fn fmt (&self, stream: &mut fmt::Formatter) -> fmt::Result {
        write!(stream, "Person<id: {}, ssn: {}>", self.id, self.ssn)
    }
}

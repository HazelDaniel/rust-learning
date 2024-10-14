use std::fmt;

#[derive(Debug)]
struct Object1(i32);

#[derive(Debug)]
struct TwoNumTuple(i32, i32);

impl fmt::Display for TwoNumTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{}, {}]", self.0, self.1);
    }
}

fn main() {
    /// # this is the entry point into the program
    println!("hello rust!");
    println!("object is {:#?}", Object1(3));

    let numTuple1 = TwoNumTuple(20, 20);

    println!("here are two 20s for you {}", numTuple1);
}
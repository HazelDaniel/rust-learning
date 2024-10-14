use std::fmt;

#[derive(Debug)]
struct Object1(i32);

#[derive(Debug)]
struct TwoNumTuple(i32, i32);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for TwoNumTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{}, {}]", self.0, self.1);
    }
}

impl fmt::Display for Point2D {
    fn fmt (&self, stream: &mut fmt::Formatter) -> fmt::Result {
        return write!(stream, "{} + {}i", self.x, self.y);
    }
}

fn main() {
    /// # this is the entry point into the program
    println!("hello rust!");
    println!("object is {:#?}", Object1(3));

    let numTuple1 = TwoNumTuple(20, 20);

    let point1 = Point2D {x: 3.3, y: 7.2};

    println!("here are two 20s for you {}", numTuple1);

    // ACTIVITY
    println!("Debug: Complex {{ real: {}, imag: {} }}", point1.x, point1.y);
    println!("Display: {}", point1);
}

#![allow(unused)]
#![allow(dead_code)]

use std::fmt;

fn main () {
    // handle_fizzbuzz();
    handle_cartesian();
    return ();
}

fn handle_fizzbuzz () -> () {
    fn fizzbuzz_to (n: u32) -> () {
        for i in 1..=n {
            fizzbuzz(i);
        }
    }

    fn fizzbuzz (n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz!");
        }
        else if is_divisible_by(n, 5) {
            println!("buzz!");
        }
        else if is_divisible_by(n, 3) {
            println!("fizz!");
        }
        else {
            println!("{n}");
        }
    }

    fn is_divisible_by (x: u32, y: u32) -> bool {
        if y == 0 {
            return false;
        }

        return x % y == 0;
    }

    fizzbuzz_to(100);
}


fn handle_cartesian () -> () {
    /*
    * - implement a Rectangle struct with two fields; p1, p2. each should be a Point instance
    * - implement the Point struct to have fields; x, y (both f64)
    * - on the Point struct, implement two associated functions; origin, new. the origin returns a Point instance
    *       with coordinates set to (0, 0). while the 'new' associated function returns a Point instance
    *       with coordinates set to the provided x and y arguments
    * - implement three methods on the Rectangle struct; area, perimeter, translate
    *       area: takes in no extra parameters. only calculates the area of the current instance
    *       based on the p1 and p2 fields
    *
    *       perimeter: also takes in no extra parameters. only calculates the parameter of the
    *       current instance based on the p1 and p2 fields
    *
    *       translate: takes in two extra parameters; x and y respectively (both f64) -translating
    *       the p1 and p2 fields by x and y respectively. note that the instance calling this method has
    *       to be a mutable instance.
    *
    * - create a two-tuple struct called Pair the entries should be dynamically allocated i32
    * integers (hint: use the Box<T> construct). implement the 'destroy' method for it
    *
    */

    struct Point {
        x: f64,
        y: f64
    }

    struct Rectangle {
        p1: Point,
        p2: Point
    }

    impl Point {
        fn origin () -> Point {
            return Point {x: 0.0, y: 0.0};
        }
        
        fn new (x: f64, y: f64) -> Point {
            return Point {x, y};
        }
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            let Point {x: x1, y: y1} = self.p1;
            let Point {x: x2, y: y2} = self.p2;

            return ((x2 - x1) * (y2 - y1)).abs();
        }

        fn perimeter(&self) -> f64 {
            let Point {x: x1, y: y1} = self.p1;
            let Point {x: x2, y: y2} = self.p2;

            return 2.0 * ((x2 - x1).abs() * (y2 - y1).abs());
        }

        fn translate(&mut self, x: f64, y: f64) -> () {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    impl fmt::Display for Rectangle {
        fn fmt (&self, stream: &mut fmt::Formatter) -> fmt::Result {
            return write!(stream, "Rectangle<({}, {}) => ({}, {})>", self.p1.x, self.p1.y, self.p2.x, self.p2.y);
        }

    }

    struct Pair (Box<i32>, Box<i32>);

    impl Pair {
        fn destroy (self) -> () {
            let Pair (x_coordinate, y_coordinate) = self;

            println!("destroying Pair ({x_coordinate}, {y_coordinate})");
        }
    }

    fn run () -> () {
        let first_rect = Rectangle {p1: Point::origin(), p2: Point::new(0.5, -0.5)};

        println!("the area of the first rectangle is {}", first_rect.area());
        println!("the perimeter of the first rectangle is {}", first_rect.perimeter());

        let mut second_rect = Rectangle {p1: Point::new(2.5, 15.05), p2: Point::new(0.8, 12.0)};

        println!("the area of the second rectangle is {}", second_rect.area());
        println!("the perimeter of the second rectangle is {}", second_rect.perimeter());

        println!("a second rectangle created: {}", second_rect);
        second_rect.translate(2.0, 5.0);
        println!("the area of the second rectangle stays the same after translation {}", second_rect.area());
        println!("but its coordinates changed: {}", second_rect);

        let dynamic_pair = Pair(Box::new(2), Box::new(5));
        dynamic_pair.destroy();
    }

    run();
}

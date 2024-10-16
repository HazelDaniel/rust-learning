#![allow(dead_code)]
#![allow(unused)]

use std::fmt;

#[derive(fmt::Debug)]
// structs with integer fields
struct Point {
    x: i16,
    y: i16,
}

#[derive(fmt::Debug)]
// unit structs
struct Unit;

#[derive(fmt::Debug)]
// nested structs - structs with struct fields
struct Diagonal {
    top_left: Point,
    bottom_right: Point,
}

#[derive(fmt::Debug)]
struct Pair(i16, i16);

fn main () {
    // declaration
    let _pair: Pair = Pair (4, 3);

    let Pair (x, y) = _pair;
    let point: Point = Point {x, y};

    let Point {x: top_left, y} = point;
    // destructuring
    println!("top left : {} \t bottom right: {}", top_left, y);

    // spread (struct update) syntax
    let bottom_right: Point = Point {x: -2, ..point};

    let diag: Diagonal = Diagonal {
        top_left: point,
        bottom_right,
    };

    let _unit: Unit = Unit;

    return ();
}

#![allow(dead_code)]
#![allow(unused)]

use std::fmt;

const LANGUAGE: &str = "rust";
static THRESHOLD: i32 = 320;

#[derive(fmt::Debug)]
struct MouseCoordinates (i32, i32);

enum MouseEvent {
    Hover,
    Click,
    MouseMove {x: i32, y: i32},
    Drag {x: i32, y: i32},
    Drop {data: String},
    Coordinates (i32, i32),
}

// with implicit discriminator
enum Number {
    Zero,
    One,
    Two,
}

impl fmt::Display for Number {
    fn fmt(&self, stream: &mut fmt::Formatter) -> fmt::Result {
        match (&self) {
            Number::Zero => return write!(stream, "Number<{}>", 0 as u32),
            Number::One => return write!(stream, "Number<{}>", 1 as u32),
            Number::Two => return write!(stream, "Number<{}>", 2 as u32),
        }
    }
}

// with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn handle (event: MouseEvent) {
    use MouseEvent::{MouseMove, Drag}; // using 'use' to avoid manual namespacing/scoping
    match event {
        MouseEvent::Hover => println!("mouse hovered!"),
        MouseEvent::Click => println!("event was a click!"),
        MouseEvent::Drop {data} => println!("data dropped: {}", data),
        MouseEvent::Coordinates(x, y) => println!("current mouse coordinates ({x}, {y})"),
        MouseMove {x, y} => println!("mouse moving from x={x}, y={y}"),
        Drag {x, y} => println!("dragging from x={x}, y={y}"),
    }
}

fn main () {
    let on_hover = MouseEvent::Hover;
    let on_click = MouseEvent::Click;
    let on_mousemove = MouseEvent::MouseMove {x: 3, y: 4};
    let on_drag = MouseEvent::Drag {x: 0, y: -100};
    let on_drop = MouseEvent::Drop {data: String::from("svg+xml")};
    let on_coordinates = MouseEvent::Coordinates (2, 8);

    handle(on_hover);
    handle(on_click);
    handle(on_mousemove);
    handle(on_drag);
    handle(on_drop);
    handle(on_coordinates);

    println!("zero is {}", Number::Zero);

    return ();
}


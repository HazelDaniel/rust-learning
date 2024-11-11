#![allow(unused)]
use std::marker::{Copy, PhantomData};
use std::ops::Add;
use std::fmt;

#[derive(fmt::Debug, Clone, Copy)]
struct Inch {}
#[derive(fmt::Debug, Clone, Copy)]
struct Mm {}

#[derive(fmt::Debug, Clone, Copy)]
struct Length<U>(f64, PhantomData<U>);

impl<Unit> Add for Length<Unit> {
    // this makes sure that only instances of phantom type Unit can be added together
    type Output = Length<Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main () -> () {
    println!("welcome to phantom types");
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let three_feet = (one_foot + one_foot) + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot + one_foot = {:?} in", three_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    return ();
}

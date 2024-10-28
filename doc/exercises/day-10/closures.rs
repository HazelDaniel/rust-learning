#![allow(dead_code)]
#![allow(unused)]

use std::mem;

fn call_with_20<F> (func: F) -> () where
    F: Fn(i32) -> () {

    func(20)
}

fn return_with_num<F> (func: F, num: i32) -> i32 where
    F: Fn(i32) -> i32 {

    func(num)
}

fn use_with_drop<T>(func: T) -> () where
    T: FnOnce()-> () {

    func()
}

fn main () -> () {
    let logger = |x| println!("i got called with {:?}", x);
    call_with_20(logger);

    let num_function = |x| x;
    let doubler = |x| 2 * x;

    let x = return_with_num(num_function, 11);
    let y = return_with_num(doubler, 11);

    println!("the mysterious number is {}", x);
    println!("the doubled number is {}", y);

    let movable_string = "property";
    let copied_string = movable_string.to_owned();

    let mover_fn = move || {
        println!("as a new owner of {}, i will be moving it.", {movable_string});
    };

    use_with_drop(mover_fn);

    mem::drop(mover_fn);

    println!("forget about accessing {}", movable_string);
    println!("but you can still reach a copy of {} if you want", copied_string);

    return ();
}

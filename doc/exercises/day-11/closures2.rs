#![allow(dead_code)]
#![allow(unused)]

// a function that returns a closure
fn curry_2_addition () -> impl Fn (i32) -> i32 {
    move |x| x + 2
}

// a function that takes in anything that can be bounded by `Fn` trait
fn wrapper <F: Fn()> (func: F) -> () {
    func();
}

// could also be defined using this signature
/* fn wrapper <F> (func: F) -> () where
    F: Fn () -> () {
    func();
} */

fn newspaper () -> () {
    println!("here's your news!");
}


fn main () -> () {
    let curried_15_func = curry_2_addition();
    let curried_15 = curried_15_func(13);

    println!("curried 15 is infact {}", curried_15);

    let news_closure = || println!("here's your news! (if it brings you any closure)");

    wrapper(newspaper);
    wrapper(news_closure);

    return ();
}

#![allow(dead_code)]
#![allow(unused)]
#![allow(non_snake_case)]

pub fn compose_with_integer <T, U> (input: i32, func1: T, func2: U) -> i32 where
    T: Fn (i32) -> i32,
    U: Fn (i32) -> i32 {
    func2(func1(input))
}

#![allow(dead_code)]
use std::fmt;

struct Object<T> {
    content:    T
}

fn gen_return<T> (input: T) -> T {
    input
}

struct GenVal<T> {
    gen_val:    T,
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        let ref x = self.gen_val;
        x
    }
}

trait DoubleDrop<T> {
    fn drop_both(self, _: T) -> ();
}

trait OptionalDebug {
    fn optional_print(self);
}

impl<U> OptionalDebug for U where
    Option<U>: fmt::Debug {
    fn optional_print(self) {
        println!("i am printable. {:?}", Some(self));
    }
}

impl<T, U> DoubleDrop<T> for U {
    fn drop_both(self, _param: T) -> () {}
}


// newtype idiom
#[derive(fmt::Debug)]
struct Age(u8);

trait AgeInterface {
    fn to_age(input: u8) -> Age;
}

impl TryFrom<u8> for Age {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Age(value))
    }
}

trait ToAge {
    fn to_age(self) -> Age;
}

impl ToAge for u32 {
    fn to_age(self) -> Age {
        Age(self as u8)
    }
}

fn main () -> () {
    let text = Object {content: "hello"};
    let status_code = Object {content: 200};

    println!("the content of the text is {}", text.content);
    println!("the status code is {}", status_code.content);

    let returned_number_generic = gen_return::<i32>("2".parse::<i32>().unwrap());
    let returned_string_generic = gen_return::<String>(String::from("2 thousand"));

    println!("the returned number generic is {returned_number_generic}");
    println!("the returned string generic is {:?}", returned_string_generic);

    let y = GenVal { gen_val: 3_i32 };
    println!("{}", y.value());

    let x = ();
    y.drop_both(x);

    #[derive(fmt::Debug)]
    struct SomeUnit {}

    let w = SomeUnit {};
    w.optional_print();

    let vec = vec![1, 2, 3];
    vec.optional_print();

    let z: Option<i32> = None;

    let optional_z: Option<i32> = match z {
        Some(x) => Some(x),
        None => None
    };

    optional_z.optional_print();

    let opt_age_num: Option<Age> = 2_u8.try_into().ok().take();

    println!("number converted into optional age is: {:?}", opt_age_num);

    let age_num = 2_u32.to_age();

    println!("number converted into age is: {:?}", age_num);

    return ();
}

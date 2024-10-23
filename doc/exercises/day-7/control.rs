#![allow(unused)]

fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    println!("names: {:?}", names);

    // into_iter consumes the array thereby making it not available for the next iteration
    /* for name in names.into_iter() {
        println!("name: {:?}", {name}) // notice how the current consumed address isn't
    // dereferenced
    } */

    // iter goes through the array collection and borrows each elements in the array - making it
    // untouched and available for the next access
    for name in names.iter() {
        println!("name: {:?}", {*name})
    }

    // iter_mut goes through the array collection and borrows `mutably` each elements in the array
    // - making it available for modification
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);



    // -----------------------------------



    let reference = &4;

    match reference {
        &ref val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref mut _is_a_reference = 3;

    match _not_a_reference {
        ref r => {
            println!("this will never print, {}", *r);
        },
        _ => println!("not a reference"),
    }

    match *_is_a_reference {
        ref mut r => {
            println!("_is_a_reference1 is a reference to a value: {:?}", *r);
            *r = 99;
        },
    }

    match _is_a_reference {
        &mut ref mut r => {
            println!("74: _is_a_reference is a reference to a value: {:?}", r);
            *_is_a_reference = 3;
        },
    }

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("86: Got a reference to a value: {:?}", *r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", *m);
        },
    }



    // -----------------------------------



    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    return ();
}


#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused)]

fn loop_to_5 () {
    let mut counter = 0_u32;
    loop {
        counter += 1;
        if counter == 3 {
            println!("here's three");
            continue;
        }
        if counter == 5 {
            println!("got to 5!, let's stop");
            break;
        }
        println!("{counter}");
    };
}

fn double_loop () {
    'outer: loop {
        'inner: loop {
            println!("inner loop only printed.");
            break 'outer; // since outer is broken out of, inner won't run again obv
        };
        println!("this will never be reached!");
    };
}

// returning from loops
fn loop_and_return_20 () -> u32 {
    let mut counter = 0_u32;

    return {
        let result: u32 = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        result
    };
}

fn main () {
    loop_to_5();
    double_loop();

    let result = 20;
    assert_eq!(result, loop_and_return_20());
}

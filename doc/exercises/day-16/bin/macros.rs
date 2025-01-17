macro_rules! create_fn  {
    ($fn_name: ident) => {
        fn $fn_name () -> () {
            println!("you called {}!", stringify!($fn_name));
        }
        
    };
}

macro_rules! print_res {
    ($expression: expr) => {
        println!("{:?} is equal to  {:?}", stringify!($expression), $expression);
    };
}

macro_rules! print_block {
    ($block_: block) => {
        println!("the current block => {:?}", stringify!($block_));
    };
}

macro_rules! test {
    ($left:expr, <and> $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    ($left:expr, <or> $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right);
    };
}

macro_rules! list_seq {
    ($x: expr) => {
        println!("{:?}", stringify!($x))
    };
    ($x: expr, $($y: expr),+) => {
        print!("{:?}, ", stringify!($x));
        list_seq!($($y),+)
    }
}

create_fn!(foo);

macro_rules! calculate {
    (eval $i: expr) => [{
        let int_val: usize = $i;
        println!("{} = {}", stringify!($i), int_val);
    }];
    (eval $i: expr, $(eval $j: expr),+) => [{
        calculate!(eval $i);
        calculate!($(eval $j),+);
    }];
}


fn main () -> () {
    foo();

    print_block!({
        print_res!({
            let x = 5;
            x * x - 2
        });
    });
    print_res!({
        let x = 5;
        x * x - 2
    });

    test!(2 == 3, <or> 4 == 4);
    test!(2 == 3, <and> 4 == 4);

    list_seq!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 3_i32);

    calculate![
        eval 2 * 2 + 7,
        eval 3,
        eval 4
    ];

    return ();
}

/// this is the start of the application
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    /// this is awesome i guess
    /// # Another program title
    // eprintln!("Hello World");
    println!("Hello {}", "Joe");
    println!("{0}, meet {1}. {1}, meet {0}.", "Joe", "Jane");
    println!("I paid ${amount_dollars}.{amount_cents} for this", amount_dollars=30, amount_cents=20);
    println!("Infact, I paid ${amount_dollars:0<5}.{amount_cents} for this", amount_dollars=30, amount_cents=20);
    println!("Base 10:               {}",   69420); //69420
    println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); //207454
    println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C
    //

    // this WILL raise error:
    // struct Unprintable(i32);
    // println!("your unprintable struct is {:?}", Unprintable(3));

    // this however, WON'T
    #[derive(Debug)]
    struct Printable(i32);
    println!("your printable struct is {:?}", Printable(3));

    let name = "Daniel";
    let age = 27;
    let daniel = Person { name, age };

    // Pretty print
    println!("{:#?}", daniel);

}

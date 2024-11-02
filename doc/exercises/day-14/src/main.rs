fn main() {
    #[cfg(not(target_os = "linux"))]
    println!("This should work on windows only");

    #[cfg(target_os = "linux")]
    println!("This code can only run on linux");

    if cfg!(target_endian = "little"){
        println!("and this is running on a small endian machine");
    } else {
        println!("and this is running on a big endian machine");
    }
}

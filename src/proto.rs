use crate::func;
use std::fs::File;
use std::io::Write;

#[unsafe(no_mangle)]
pub extern "system" fn Pick() {

    println!("{}", lc!("Calling public hello fn from func.rs"));
    func::hello();
    println!("{}", lc!("Calling private hello fn from proto.rs"));
    hello();
    println!("{}", lc!("Calling public helper fn from func.rs"));
    func::helper();
    
    // Modify file writing to include counter
    write_to_file();
}

fn write_to_file() {
    let mut file = File::create("output.txt").expect("Unable to create file");
    file.write_all(b"Hello, world!").expect("Unable to write data");
}

fn hello() {
    println!("Hello, world!");
}
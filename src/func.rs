//function to print hello world
#[unsafe(no_mangle)]
pub extern "system" fn hello() {
    println!("{}", lc!("Hello, world!"));
}

pub extern "system" fn helper() {
    private_hello();
}

fn private_hello() {
    println!("{}", lc!("Hello, world!"));
}
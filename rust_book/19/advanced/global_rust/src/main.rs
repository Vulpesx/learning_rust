//imutable static is safe, mutable is not but may be usefull, also static variables can't duplicate
//constants can duplicate and move memory adress but are always imutable
static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

fn main() {
    println!("name is: {}", HELLO_WORLD);

    add_to_counter(3);

    unsafe {
        println!("counter is: {}", COUNTER);
    }
}

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

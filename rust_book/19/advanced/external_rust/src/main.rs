fn main() {
    println!("read dont run!");
}

// bring functions from a binary that uses C ABI (how function is shown in binary)
// ABI - Aplication Binary Interface. used for calling binary files
extern "C" {
    fn call_to_c(input: i32) -> i32;
}

// making a function usable by other languages "C" is the ABI we will use, it is the most common
// no_mangle stops the compiler from mangling the name
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("this is rust used in C");
}

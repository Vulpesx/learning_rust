
struct SP {
    data: String,
}

impl Drop for SP {
    fn drop (&mut self) {
        println!("Droping SP with data: {}", self.data);
    }
}

fn main() {
    let c = SP { data: String::from("my stuff") };
    let d = SP { data: String::from("other stuff") };
    println!("SP's created :)");
    drop(c);
    println!("SP dropped before end of scope");
}


enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    //creating a Box
    // let b = Box::new(5);
    // You can acces teh value stored in the box
    // as you would with the value it self
    // println!("b = {}", b);
    
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

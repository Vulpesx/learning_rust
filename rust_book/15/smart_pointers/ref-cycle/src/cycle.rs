use std::{cell::{RefCell, Ref}, rc::Rc};
use crate::cycle::List::{Cons, Nil};


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    
    fn newRc(a: i32, b: RefCell<Rc<List>>) -> Rc<List> {
        Rc::new(Cons(a, b))
    }

    fn new(a: i32, b: List) -> RefCell<Rc<List>> {
        RefCell::new(Rc::new(b))
    }

    fn newNil() -> RefCell<Rc<List>> {
        RefCell::new(Rc::new(Nil))
    }

    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn run() {
    let a = List::newRc(5, List::newNil());

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = List::newRc(10, RefCell::new(Rc::clone(&a)));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&a);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    //Uncomment the next line to see that we have a cycle;
    //it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

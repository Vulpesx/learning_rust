use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn new(v: i32) -> Rc<Node> {
        Rc::new(Node {
            value: v,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new())
        })
    }
    
    fn add_child(s: &Rc<Node>, n: Rc<Node>) {
        s.children.borrow_mut().push(n);
        s.children.borrow().last().unwrap().set_parent(Rc::downgrade(s));
    }

    fn set_parent(&self, p: Weak<Node>) {
        *self.parent.borrow_mut() = p;
    }
}

pub fn run() {
    let leaf = Node::new(3);
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf));

    let branch = Node::new(5);
    Node::add_child(&branch, Rc::clone(&leaf));
    println!(
        "branch strong = {} weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch));

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf));

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf));
}

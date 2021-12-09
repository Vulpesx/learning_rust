use std::{cell::RefCell, borrow::BorrowMut};

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let p = self.value as f64 / self.max as f64;  

        if p >= 1.0 { self.messenger.send("Error: You are over your daily quota!"); }
        else if p >= 0.9 { self.messenger.send("Urgent Warning: You've used over 90% of your daily quota!"); }
        else if p >= 0.75 { self.messenger.send("Warning: You've used up over 75% of your daily quota!"); }
        
    }
}

fn main () {
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mocky {
        messages: RefCell<Vec<String>>,
    }

    impl Messenger for Mocky {
        

        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(String::from(msg));
        }
    }

    impl Mocky {
        fn new() -> Mocky {
            Mocky {
                messages: RefCell::new(vec![])
            }
        }
    }

    #[test]
    fn over_75() {
        let m = Mocky::new(); 
        let mut lt = LimitTracker::new(&m, 100);

        lt.set_value(80);

        assert_eq!(m.messages.borrow().len(), 1);
    }
}


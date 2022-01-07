use std::{cell::{RefCell, Ref}, borrow::Borrow};

pub struct Post {
    state: Option<Box<dyn State>>,
    content: RefCell<String>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: RefCell::new(String::new()),
        }
    }

    pub fn add_text(&self, text: &str) {
        let s = self.state.as_ref().unwrap();
        s.add_text(self, text);
    }

    pub fn content(&self) -> &str {
        let s = self.state.as_ref().unwrap().content(self).clone();
        &s
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content(&self, post: &Post) -> String {
        String::new()
    }
    fn add_text(&self, post: &Post, text: &str) {
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approves: RefCell::new(0)})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn add_text(&self, post: &Post, text: &str) {
        (*post.content.borrow_mut()).push_str(text);
    }
}

struct PendingReview {
    approves: RefCell<u8>,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if *self.approves.borrow() >= 2 {
            return Box::new(Published {});
        }
        let n = self.approves.take();
        *self.approves.borrow_mut() = n + 1;
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content(&self, post: &Post) -> String{
        let s = post.content.clone().take();
        s
    }
}

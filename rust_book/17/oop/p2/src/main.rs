use p2::gui::{Screen, Button, Draw};

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {

    fn draw(&self) {
        println!("Imagine a selection box {} wide, {}, high and {:?} options",
                 self.width,
                 self.height,
                 self.options);
    }
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> SelectBox {
        SelectBox {
            width,
            height,
            options,
        }
    }
    
}

fn main() {
    let mut screen = Screen::new();

    screen.add(Box::new(Button::new(50, 10, String::from("test"))));
    screen.add(Box::new(SelectBox::new(75, 10, vec![
        String::from("yes"),
        String::from("maybe"),
        String::from("no"),
    ])));

    screen.run();
}

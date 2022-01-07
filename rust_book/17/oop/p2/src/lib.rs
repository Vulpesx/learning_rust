pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn new() -> Screen {
            Screen {
                components: vec![],
            }
        }

        pub fn add(&mut self, value: Box<dyn Draw>) {
            self.components.push(value);
        }

        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        
        fn draw(&self) {
            println!("Imagine a buttone {} wide, and {} high with a label of {}",
                     self.width,
                     self.height,
                    self.label);
        }
    }

    impl Button {
        pub fn new(width: u32, height: u32, label: String) -> Button {
            Button {
                width,
                height,
                label,
            }
        }
    }
}

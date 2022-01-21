use std::{ops::Add, fmt};

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "-".repeat(len + 4));
        println!("|{}|", " ".repeat(len + 2));
        println!("| {} |", output);
        println!("|{}|", " ".repeat(len + 2));
        println!("{}", "-".repeat(len + 4));
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    let a = Millimeters(100);
    let b = Meters(5);

    println!("{:?}", a + b);

    let p = Point{ x: 3, y: 5};
    p.outline_print();
}


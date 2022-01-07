//encapsulation
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(list: Vec<i32>) -> AveragedCollection {
        let mut a = AveragedCollection {
            list,
            average: 0f64,
        };
        a.update_average();
        a
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_average();
                Some(v)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut a = AveragedCollection::new(vec![10,-3,15,50]);
    println!("{}", a.average());
    a.add(500);
    println!("{}", a.average());
}

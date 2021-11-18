use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

// closure example in the form of a crappy workout generator
pub fn run() {
    //definately real values not faked at all
    let specified_value = 10;
    let random_number = 7;

    //super complicated stuff
    generate_workout(specified_value, random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    //create a mutable Cacher and store expensive closure
    //closure takes a number of an infered type and reurns num of same type
    //more efficient to use a function as they do not capture the environment
    //closures capture the environment in memory so they are less efficient
    fn calc(num: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }
    let mut cacher = Cacher::new(calc);
    // let mut cacher = Cacher::new(|num| {
    //     println!("Calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // });

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to say hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

//create structure to store closure and results to mitigate usage of expensive calculations
struct Cacher<T, N> where N: Eq + Hash + Copy + Clone, T: Fn(N) -> N {
    calculation: T,
    values: HashMap<N, N>
}

impl <T, N> Cacher<T, N> where N: Eq + Hash + Copy + Clone, T: Fn(N) -> N {

    fn new(n: T) -> Cacher<T, N> {
        Cacher {
            calculation: n, 
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: N) -> N {
        //gets value acording to arg if arg was already used
        //if not runs calculation and stores result
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2)
}


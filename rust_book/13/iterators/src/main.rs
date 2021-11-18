mod shoe;
mod custom;

fn main() {
    //complicated stuff
    shoe::run();
    custom::run();

    println!("Other things:");
    basic();
    methods();
}

fn basic() {
    //creating a vector (collection)
    let v = vec![1, 2, 3];
    //creating an iterator from the collection to interface with contents
    //iterator are lazy so they wont do anything until we tell them to
    //iterators are seperate from the collection and just interface with them
    let v_iter = v.iter();
    //using the iterator
    //we could have just done '''for v in v.iter()'''
    for v in v_iter {
        println!("{}", v);
    }
}

fn methods () {
    //sum (consumes iterator)
    {
        //creating collection
        let v = vec![1, 2, 3];
        //creating iterator
        let v_iter = v.iter();
        //getting the sum of the values
        //this consumes the iterator
        //and tere for it can no longer be used
        let total: i32 = v_iter.sum();
    }
    //map (produces another iterator)
    {
        //creating collection
        let v = vec![1, 2, 3];
        //creating iterator
        let v_iter = v.iter();
        //maping the values to new iterator (we canm mess with them)
        //we must collect it if we want the values wich we store in a vector, also if we dont it wont actualy do anything
        let map: Vec<_> = v_iter.map(|x| x + 1).collect();
    }
}


struct Pancakes;

fn main() {
    let v: Vec<u32> = my_vec![1, 2, 3];

    // let v = {
    //     let mut temp_vec = Vec::new();
    //     temp_vec.push(1);
    //     temp_vec.push(2);
    //     temp_vec.push(3);

    //     temp_vec
    // };

    println!("{:?}", v);

    Pancakes::hello_macro();
}

//vec! DIY
//#[macro_export] exports the macro to be available outside the crate, otherwise it would be
//imposible to be brought into scope
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*

            temp_vec
        }
    };
}

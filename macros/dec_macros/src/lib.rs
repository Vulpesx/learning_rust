#[macro_export]
macro_rules! vecy {
    [ $( $x:expr ),* ] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    use super::vecy;
    #[test]
    fn it_works() {
        let result = vecy![1, 2, 3];
        assert_eq!(result, [1, 2, 3]);
    }
}

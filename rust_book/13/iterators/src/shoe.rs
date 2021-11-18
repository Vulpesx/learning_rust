
pub fn run() {
    println!("use the tests dummy");
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 13,
                style: String::from("sandle")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ];

        let results = shoes_in_size(shoes, 10);

        assert_eq!(results,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );

    }
}

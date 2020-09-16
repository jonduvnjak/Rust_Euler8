use std::io;

fn main() {
//Figure out a way so that if the user does not type in digits then they are given an opportunity to type in digits again.
//Figure out a way to allow the user to create the window size
//guessing game has some ineresting code that might be useful
    let mut digits = String::new();
    io::stdin().read_line(&mut digits);
    let digits = string_to_digits(&digits.trim());
    
    println!("{:?}",digits );
    
    let digits_vec = maybe_vec_digits(digits);
    let window_size = 13;
    let mut largest_product = 0;

    for window in digits_vec.windows(window_size) {

        let window_product = window.iter().product::<u64>();

        if window_product > largest_product {
            largest_product = window_product;
        }
    }
    println!("{}", largest_product);

    fn maybe_vec_digits(option: Option<Vec<u64>>) -> Vec<u64> {
        match option {
            Some(vec) => vec,
            None => {
                println!("You failed. Please type in digits only.");
                Vec::new()
            }
        }
    }

    fn string_to_digits(string: &str) -> Option<Vec<u64>> {
        let mut vector = Vec::new();
        for chr in string.chars() {
            let x = chr.to_digit(10); //to_digit returns Some(u32) if char = 0-9 (using radix 10) or None if none.
            match x {
                Some(d) => vector.push(d as u64), //if x has Some(value)then push the value to vector.
                None => return None, //if x is a None value then the function stops immediately and returns None
            }
        }
        Some(vector)
    }
}

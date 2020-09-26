use std::io;

fn main() {
    let window_size = 13;

    let user_input = get_user_input();
    println!("{}", calculate_largest_window(window_size, user_input));
}


//THIS FUNCTION CALCULATES THE WINDOW OF N SIZE WITH THE LARGEST PRODUCT IN A GIVEN SET OF NUMBERS
fn calculate_largest_window(window_size: i32, digits_vec: Vec<u64>) -> u64 {
    let mut largest_product = 0;

    for window in digits_vec.windows(window_size as usize) {
        let window_product = window.iter().product::<u64>();

        if window_product > largest_product {
            largest_product = window_product;
        }
    }
    largest_product
}

//ALL BELOW IS TO 1. GET USER INPUT 2. ATTEMPT TO CONVERT IT TO VECTOR OF u64's 3. CHECK WHETHER THE CONVERSION WAS SUCCESSFUL - 
//IF NOT SUCCESSFUL THEN PROMPT USER TO TYPE IN NUMBERS AND TRY AGANIN
fn get_user_input() -> Vec<u64> {
    let mut digits_vec = Vec::new();
    let mut digits;

    while digits_vec.is_empty() {
        let mut string_digits = String::new();
        io::stdin().read_line(&mut string_digits);
        digits = string_to_digits(string_digits.trim());
        digits_vec = maybe_vec_digits(digits);
    }
    digits_vec
}

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

fn main() {
    let digits = string_to_digits("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");
    let digits_vec = maybe_vec_digits(digits);
    let window_size = 13;
    let mut largest_product = 0;
    
    for i in 0..&digits_vec.len() - window_size {
        let window_vector_product = digits_vec[i..i + window_size].into_iter().product::<u64>();
        // let window_vector_iter = window_vector.into_iter();
        // let possible_largest = window_vector_iter.product::<u64>();

        if window_vector_product > largest_product {
            largest_product = window_vector_product;
        }
    }
    println!("{:?}", largest_product);

    // fn window_range_to_vector(window_range: &[u64]) -> Vec<&u64> {
    //     let mut vector = Vec::new();
    //     for x in window_range {
    //         vector.push(x);
    //     }
    //     vector
    // }

    fn maybe_vec_digits(option: Option<Vec<u64>>) -> Vec<u64> {
        //let mut vector = Vec::new();
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

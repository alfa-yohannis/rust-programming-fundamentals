fn main() {
    // -------------------------------------------
    // Loops in Rust
    // - For loop examples
    // - Iterating with immutable references
    // - Iterating with mutable references
    // -------------------------------------------

    /*
    let numbers = vec![12, 24, 36, 48, 60, 72];
    
    for index in 0..=5 {   // Range 0 to 5 inclusive
        println!("Element {} at index {} is {}", index, index, numbers[index]);
    }
    */
    
    /*
    let values = vec![12, 24, 36, 48, 60, 72];
    for value in values {
        println!("{}", value);
    }
    println!("{:?}", values);   // This line will cause an error as `values` is moved
    */
    
    /*
    let values = vec![12, 24, 36, 48, 60, 72];
    for value in values.iter() {   // Iterate over immutable references
        println!("{}", value);
    }
    println!("{:?}", values);   // `values` remains valid here
    */
    
    let mut numbers = vec![12, 24, 36, 48, 60, 72];
    for item in numbers.iter_mut() {   // Iterate over mutable references
        *item += 10;
        println!("{}", item);
    }
    println!("{:?}", numbers);
}

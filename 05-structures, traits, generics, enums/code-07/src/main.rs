// -------------------------------------------
//          Result Enum
//            - Basic Syntax and Usage
// -------------------------------------------

/* General Syntax

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

// Example 1
/*
fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    
    match denominator {
        0. => Err(String::from("Error: Cannot divide by zero")),
        _ => Ok(numerator / denominator),
    }
}

fn main() {
    println!("\n\nResult: {:?}", safe_divide(18.0, 6.0));  
    println!("Result: {:?}", safe_divide(5.0, 0.0));
    println!("Result: {:?} \n\n", safe_divide(0.0, 3.0));
}
*/

// Example 2
fn main() {
    let numbers: Vec<i32> = vec![10, 20, 30, 40, 50];

    let result = match numbers.get(7) {
        Some(value) => Ok(value),
        None => Err("Index out of bounds"),
    };

    println!("Result of the lookup: {:?}", result);
}
